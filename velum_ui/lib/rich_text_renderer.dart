import 'package:flutter/material.dart';
import 'velum_core_wrapper.dart';
import 'dart:convert';

/// Represents a text span with its content and attributes
class StyledTextSpan {
  final String text;
  final TextAttributesData? attributes;

  StyledTextSpan({
    required this.text,
    this.attributes,
  });

  /// Converts to Flutter TextStyle
  TextStyle toTextStyle({TextStyle? baseStyle}) {
    TextStyle style = baseStyle ?? const TextStyle();
    
    if (attributes != null) {
      if (attributes!.bold == true) {
        style = style.copyWith(fontWeight: FontWeight.bold);
      }
      if (attributes!.italic == true) {
        style = style.copyWith(fontStyle: FontStyle.italic);
      }
      if (attributes!.underline == true) {
        style = style.copyWith(
          decoration: (style.decoration ?? TextDecoration.none) | TextDecoration.underline,
        );
      }
      if (attributes!.fontSize != null) {
        style = style.copyWith(fontSize: attributes!.fontSize!.toDouble());
      }
      if (attributes!.fontFamily != null) {
        style = style.copyWith(fontFamily: attributes!.fontFamily);
      }
      if (attributes!.foreground != null) {
        try {
          style = style.copyWith(
            color: Color(int.parse(attributes!.foreground!.substring(1), radix: 16) | 0xFF000000),
          );
        } catch (e) {
          // Invalid color, skip
        }
      }
      if (attributes!.background != null) {
        try {
          style = style.copyWith(
            backgroundColor: Color(int.parse(attributes!.background!.substring(1), radix: 16) | 0xFF000000),
          );
        } catch (e) {
          // Invalid color, skip
        }
      }
    }
    
    return style;
  }
}

/// Parses the JSON output from getTextWithAttributes
List<StyledTextSpan> parseStyledText(String json) {
  final List<StyledTextSpan> spans = [];
  
  try {
    final List<dynamic> items = jsonDecode(json);
    
    for (var item in items) {
      final text = item['text'] as String? ?? '';
      final attrsJson = item['attrs'];
      
      TextAttributesData? attrs;
      if (attrsJson != null && attrsJson != 'null') {
        attrs = TextAttributesData(
          bold: attrsJson['bold'] as bool?,
          italic: attrsJson['italic'] as bool?,
          underline: attrsJson['underline'] as bool?,
          fontSize: attrsJson['font_size'] as int?,
          fontFamily: attrsJson['font_family'] as String?,
          foreground: attrsJson['foreground'] as String?,
          background: attrsJson['background'] as String?,
        );
      }
      
      spans.add(StyledTextSpan(text: text, attributes: attrs));
    }
  } catch (e) {
    // If parsing fails, return a single span with no attributes
    spans.add(StyledTextSpan(text: json));
  }
  
  return spans;
}

/// A widget that displays rich text with styles
class VelumRichText extends StatefulWidget {
  final String text;
  final List<StyledTextSpan> spans;
  final TextStyle? baseStyle;
  final SelectionInfo? selection;
  final Color? selectionColor;
  final double fontSize;

  const VelumRichText({
    super.key,
    required this.text,
    this.spans = const [],
    this.baseStyle,
    this.selection,
    this.selectionColor,
    this.fontSize = 16.0,
  });

  /// Creates VelumRichText from plain text (no styling)
  factory VelumRichText.plainText(
    String text, {
    TextStyle? style,
    double fontSize = 16.0,
  }) {
    return VelumRichText(
      text: text,
      spans: [StyledTextSpan(text: text)],
      baseStyle: style,
      fontSize: fontSize,
    );
  }

  /// Creates VelumRichText from JSON output of getTextWithAttributes
  factory VelumRichText.fromJson(
    String json, {
    TextStyle? baseStyle,
    double fontSize = 16.0,
  }) {
    return VelumRichText(
      text: '',
      spans: parseStyledText(json),
      baseStyle: baseStyle,
      fontSize: fontSize,
    );
  }

  @override
  State<VelumRichText> createState() => _VelumRichTextState();
}

class _VelumRichTextState extends State<VelumRichText> {
  List<TextSpan> _buildTextSpans() {
    final spans = widget.spans;
    
    if (spans.isEmpty || (spans.length == 1 && spans[0].text == widget.text)) {
      // No styling, return plain text
      return [
        TextSpan(
          text: widget.text,
          style: widget.baseStyle,
        ),
      ];
    }
    
    final List<TextSpan> textSpans = [];
    
    for (var span in spans) {
      textSpans.add(
        TextSpan(
          text: span.text,
          style: span.toTextStyle(baseStyle: widget.baseStyle),
        ),
      );
    }
    
    return textSpans;
  }

  @override
  Widget build(BuildContext context) {
    return RichText(
      text: TextSpan(
        children: _buildTextSpans(),
      ),
      textAlign: TextAlign.left,
      textDirection: TextDirection.ltr,
    );
  }
}

/// Represents the current text selection
class SelectionInfo {
  final int start;
  final int end;

  SelectionInfo({required this.start, required this.end}) {
    assert(start >= 0 && end >= 0);
  }

  bool contains(int position) {
    return position >= start && position < end;
  }

  bool isEmpty() => start == end;
}

/// A widget that renders selectable rich text
class SelectableRichText extends StatefulWidget {
  final String text;
  final List<StyledTextSpan> spans;
  final TextStyle? baseStyle;
  final Color? selectionColor;
  final double fontSize;
  final ValueChanged<SelectionInfo>? onSelectionChanged;
  final TextSelection? initialSelection;

  const SelectableRichText({
    super.key,
    required this.text,
    required this.spans,
    this.baseStyle,
    this.selectionColor,
    this.fontSize = 16.0,
    this.onSelectionChanged,
    this.initialSelection,
  });

  @override
  State<SelectableRichText> createState() => _SelectableRichTextState();
}

class _SelectableRichTextState extends State<SelectableRichText> {
  final FocusNode _focusNode = FocusNode();
  final TextEditingController _controller = TextEditingController();
  
  @override
  void initState() {
    super.initState();
    if (widget.initialSelection != null) {
      _controller.value = TextEditingValue(
        text: widget.text,
        selection: widget.initialSelection!,
      );
    } else {
      _controller.value = TextEditingValue(text: widget.text);
    }
  }

  @override
  void didUpdateWidget(SelectableRichText oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.text != widget.text) {
      final currentSelection = _controller.selection;
      _controller.value = TextEditingValue(
        text: widget.text,
        selection: currentSelection,
      );
    }
  }

  @override
  void dispose() {
    _focusNode.dispose();
    _controller.dispose();
    super.dispose();
  }

  List<TextSpan> _buildTextSpans() {
    if (widget.spans.isEmpty) {
      return [
        TextSpan(
          text: widget.text,
          style: widget.baseStyle,
        ),
      ];
    }
    
    final List<TextSpan> textSpans = [];
    
    for (var span in widget.spans) {
      textSpans.add(
        TextSpan(
          text: span.text,
          style: span.toTextStyle(baseStyle: widget.baseStyle),
        ),
      );
    }
    
    return textSpans;
  }

  void _handleSelectionChanged(TextSelection selection, SelectionChangedCause? cause) {
    widget.onSelectionChanged?.call(
      SelectionInfo(start: selection.start, end: selection.end),
    );
  }

  @override
  Widget build(BuildContext context) {
    return SelectionArea(
      focusNode: _focusNode,
      onSelectionChanged: _handleSelectionChanged,
      child: RichText(
        text: TextSpan(
          children: _buildTextSpans(),
        ),
        textAlign: TextAlign.left,
        textDirection: TextDirection.ltr,
      ),
    );
  }
}
