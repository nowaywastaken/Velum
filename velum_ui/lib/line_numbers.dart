import 'package:flutter/material.dart';

/// A widget that displays line numbers for the editor gutter.
///
/// This widget shows line numbers in a scrollable column that syncs with
/// the editor content. It highlights the current line and provides
/// fixed-width formatting for consistent alignment.
class LineNumberGutter extends StatelessWidget {
  /// The total number of lines in the document
  final int lineCount;

  /// The currently highlighted line (1-indexed)
  final int currentLine;

  /// Optional scroll controller to sync scrolling with the editor
  final ScrollController? scrollController;

  /// The line height in logical pixels (used for scroll sync calculation)
  final double lineHeight;

  const LineNumberGutter({
    super.key,
    required this.lineCount,
    this.currentLine = 1,
    this.scrollController,
    this.lineHeight = 20.0,
  });

  /// Calculates the width needed to display the maximum line number
  double _calculateGutterWidth() {
    if (lineCount <= 0) {
      return 40.0; // Minimum width for "0" or empty state
    }
    final maxDigits = lineCount.toString().length;
    // Each digit is approximately 12-14px in monospace font
    // Add padding for right alignment
    return (maxDigits * 14.0) + 24.0;
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final isDark = theme.brightness == Brightness.dark;

    // Colors for line number gutter
    final gutterBackground = isDark
        ? const Color(0xFF1E1E1E)
        : const Color(0xFFF5F5F5);
    final lineNumberColor = isDark
        ? const Color(0xFF858585)
        : const Color(0xFF6E6E6E);
    final currentLineBackground = isDark
        ? const Color(0xFF2D2D2D)
        : const Color(0xFFE8E8E8);
    final currentLineColor = isDark
        ? const Color(0xFFFFFFFF)
        : const Color(0xFF000000);

    return Container(
      width: _calculateGutterWidth(),
      decoration: BoxDecoration(
        color: gutterBackground,
        border: Border(
          right: BorderSide(
            color: isDark
                ? const Color(0xFF3D3D3D)
                : const Color(0xFFE0E0E0),
            width: 1.0,
          ),
        ),
      ),
      child: Scrollbar(
        controller: scrollController,
        thumbVisibility: false,
        child: ListView.builder(
          controller: scrollController,
          physics: const ClampingScrollPhysics(),
          itemCount: lineCount > 0 ? lineCount : 1,
          itemExtent: lineHeight,
          itemBuilder: (context, index) {
            final lineNumber = index + 1;
            final isCurrentLine = lineNumber == currentLine;

            return Container(
              height: lineHeight,
              padding: const EdgeInsets.symmetric(horizontal: 8.0),
              decoration: BoxDecoration(
                color: isCurrentLine ? currentLineBackground : null,
              ),
              child: Align(
                alignment: Alignment.centerRight,
                child: Text(
                  lineCount > 0 ? lineNumber.toString() : '1',
                  style: TextStyle(
                    fontFamily: 'Menlo',
                    fontFamilyFallback: ['Monaco', 'Courier', 'monospace'],
                    fontSize: 14.0,
                    fontWeight: isCurrentLine ? FontWeight.w600 : FontWeight.w500,
                    color: isCurrentLine ? currentLineColor : lineNumberColor,
                    height: 1.0,
                  ),
                  textAlign: TextAlign.right,
                ),
              ),
            );
          },
        ),
      ),
    );
  }
}

/// A wrapper widget that combines the line number gutter with an editor.
///
/// This handles the layout and scroll synchronization between the gutter
/// and the editor content.
class EditorWithGutter extends StatelessWidget {
  final Widget editor;
  final int lineCount;
  final int currentLine;
  final ScrollController? gutterScrollController;
  final double lineHeight;

  const EditorWithGutter({
    super.key,
    required this.editor,
    required this.lineCount,
    this.currentLine = 1,
    this.gutterScrollController,
    this.lineHeight = 20.0,
  });

  @override
  Widget build(BuildContext context) {
    return Row(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        LineNumberGutter(
          lineCount: lineCount,
          currentLine: currentLine,
          scrollController: gutterScrollController,
          lineHeight: lineHeight,
        ),
        Expanded(child: editor),
      ],
    );
  }
}
