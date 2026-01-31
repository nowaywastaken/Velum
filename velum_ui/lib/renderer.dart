import 'package:flutter/material.dart';
import 'dart:convert';
import 'velum_core_wrapper.dart';

class LayoutLine {
  final int lineNumber;
  final double start;
  final double end;
  final double width;
  final double height;
  final String text;

  LayoutLine({
    required this.lineNumber,
    required this.start,
    required this.end,
    required this.width,
    required this.height,
    required this.text,
  });

  factory LayoutLine.fromJson(Map<String, dynamic> json, String fullText) {
    // Note: The JSON structure from Rust's LineLayoutInfo matches this partially
    // Rust: { line_number, start, end, width, break_type, ... }
    
    final start = json['start'] as int;
    final end = json['end'] as int;
    
    // Safety check for indices
    final safeStart = start < fullText.length ? start : fullText.length;
    final safeEnd = end <= fullText.length ? end : fullText.length;
    
    return LayoutLine(
      lineNumber: json['line_number'] as int,
      start: safeStart.toDouble(),
      end: safeEnd.toDouble(),
      width: (json['width'] as num).toDouble(),
      height: 20.0, // Default for now, should come from layout config
      text: fullText.substring(safeStart, safeEnd),
    );
  }
}

class DocumentLayout {
  final List<LayoutLine> lines;
  final double totalHeight;
  final double totalWidth;

  DocumentLayout({
    required this.lines,
    required this.totalHeight,
    required this.totalWidth,
  });

  factory DocumentLayout.fromJson(Map<String, dynamic> json, String fullText) {
    // Rust: { paragraphs: [...], total_width, total_height, line_height }
    // We need to flatten paragraphs into lines for simple rendering
    
    final List<LayoutLine> lines = [];
    final paragraphs = json['paragraphs'] as List;
    
    // We need to accumulate height because Rust returns relative line numbers within paragraphs?
    // Actually Rust's LineLayoutInfo has line_number 0-based.
    // Ideally we want absolute Y positions.
    // For this prototype, we'll stack paragraphs.
    
    // Let's assume Rust returns { paragraphs: [{ lines: [...] }] }
    
    double currentY = 0;
    final lineHeight = (json['line_height'] as num).toDouble();
    
    for (var p in paragraphs) {
      final pLines = p['lines'] as List;
      for (var l in pLines) {
        // Rust LineLayoutInfo: { line_number, start, end, width, ... }
        // We construct a flattened list
        lines.add(LayoutLine.fromJson(l, fullText));
      }
    }
    
    return DocumentLayout(
      lines: lines,
      totalHeight: (json['total_height'] as num).toDouble(),
      totalWidth: (json['total_width'] as num).toDouble(),
    );
  }
}

class VelumRenderer extends StatefulWidget {
  final double width;
  
  const VelumRenderer({super.key, required this.width});

  @override
  State<VelumRenderer> createState() => _VelumRendererState();
}

class _VelumRendererState extends State<VelumRenderer> {
  DocumentLayout? _layout;
  String _fullText = "";
  bool _isLoading = false;

  @override
  void initState() {
    super.initState();
    _refreshLayout();
  }

  @override
  void didUpdateWidget(VelumRenderer oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.width != widget.width) {
      _refreshLayout();
    }
  }

  Future<void> _refreshLayout() async {
    if (_isLoading) return;
    setState(() => _isLoading = true);

    try {
      final core = VelumCoreWrapper();
      _fullText = await core.api.getFullText();
      final jsonStr = await core.layoutCurrentDocument(widget.width);
      final jsonMap = jsonDecode(jsonStr);
      
      if (mounted) {
        setState(() {
          _layout = DocumentLayout.fromJson(jsonMap, _fullText);
          _isLoading = false;
        });
      }
    } catch (e) {
      debugPrint("Layout error: $e");
      if (mounted) setState(() => _isLoading = false);
    }
  }

  @override
  Widget build(BuildContext context) {
    if (_isLoading || _layout == null) {
      return const Center(child: CircularProgressIndicator());
    }

    return CustomPaint(
      painter: TextLayerPainter(
        layout: _layout!,
        fullText: _fullText,
      ),
      size: Size(widget.width, _layout!.totalHeight),
    );
  }
}

class TextLayerPainter extends CustomPainter {
  final DocumentLayout layout;
  final String fullText;
  final Paint _textPaint;

  TextLayerPainter({required this.layout, required this.fullText})
      : _textPaint = Paint()..color = Colors.black;

  @override
  void paint(Canvas canvas, Size size) {
    // Draw background
    canvas.drawRect(Offset.zero & size, Paint()..color = Colors.white);

    // Draw lines
    // We need to calculate Y position manually since we flattened the structure
    // or use the paragraph structure.
    // For MPV, let's just stack them using a fixed line height assumption if missing
    // But wait, the JSON has total_height.
    
    // In Rust line_layout.rs:
    // ParagraphLayout has total_height.
    // We need to fetch line_height from the JSON or config. Default is 14 * 1.2 = 16.8
    // Let's hardcode 16.8 for now if dynamic measure fails, but better to read from JSON.
    
    double y = 0;
    const double approxLineHeight = 16.8; 

    final textStyle = const TextStyle(
      color: Colors.black,
      fontSize: 14.0,
      fontFamily: 'Courier',
    );
    
    final textPainter = TextPainter(
      textDirection: TextDirection.ltr,
    );

    // Simple robust rendering:
    // Iterate through lines and draw them.
    for (var line in layout.lines) {
      // Draw text
      textPainter.text = TextSpan(text: line.text, style: textStyle);
      textPainter.layout();
      textPainter.paint(canvas, Offset(0, y)); // Simple left align for now

      y += approxLineHeight;
    }
  }

  @override
  bool shouldRepaint(covariant TextLayerPainter oldDelegate) {
    return oldDelegate.layout != layout;
  }
}
