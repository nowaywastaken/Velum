import 'dart:ffi';
import 'dart:io';
import 'bridge_generated.dart';

/// TextAttributes data class for Flutter
class TextAttributesData {
  final bool? bold;
  final bool? italic;
  final bool? underline;
  final int? fontSize;
  final String? fontFamily;
  final String? foreground;
  final String? background;

  TextAttributesData({
    this.bold,
    this.italic,
    this.underline,
    this.fontSize,
    this.fontFamily,
    this.foreground,
    this.background,
  });

  factory TextAttributesData.fromString(String attrs) {
    final parts = attrs.split(',');
    if (parts.length != 7) {
      return TextAttributesData();
    }
    
    return TextAttributesData(
      bold: parts[0] == 'true' ? true : (parts[0] == 'false' ? false : null),
      italic: parts[1] == 'true' ? true : (parts[1] == 'false' ? false : null),
      underline: parts[2] == 'true' ? true : (parts[2] == 'false' ? false : null),
      fontSize: parts[3] == 'None' ? null : int.tryParse(parts[3]),
      fontFamily: parts[4] == 'None' ? null : parts[4],
      foreground: parts[5] == 'None' ? null : parts[5],
      background: parts[6] == 'None' ? null : parts[6],
    );
  }

  String toJson() {
    final Map<String, dynamic> json = {};
    if (bold != null) json['bold'] = bold;
    if (italic != null) json['italic'] = italic;
    if (underline != null) json['underline'] = underline;
    if (fontSize != null) json['font_size'] = fontSize;
    if (fontFamily != null) json['font_family'] = fontFamily;
    if (foreground != null) json['foreground'] = foreground;
    if (background != null) json['background'] = background;
    return '{"bold":${bold ?? 'null'},"italic":${italic ?? 'null'},"underline":${underline ?? 'null'},"font_size":${fontSize ?? 'null'},"font_family":${fontFamily != null ? '"$fontFamily"' : 'null'},"foreground":${foreground != null ? '"$foreground"' : 'null'},"background":${background != null ? '"$background"' : 'null'}}';
  }
}

class VelumCoreWrapper {
  static final VelumCoreWrapper _instance = VelumCoreWrapper._internal();
  late final VelumCore api;

  // Cached state for canUndo/canRedo
  bool _cachedCanUndo = false;
  bool _cachedCanRedo = false;

  // Cached selection state
  int _cachedSelectionAnchor = 0;
  int _cachedSelectionActive = 0;

  factory VelumCoreWrapper() {
    return _instance;
  }

  VelumCoreWrapper._internal() {
    final String path;
    if (Platform.isMacOS) {
      path = '../velum_core/target/debug/libvelum_core.dylib';
    } else if (Platform.isWindows) {
      path = '../velum_core/target/debug/velum_core.dll';
    } else {
      throw UnsupportedError('Unsupported platform');
    }
    
    final dylib = DynamicLibrary.open(path);
    api = VelumCoreImpl(dylib);
  }

  /// Returns the cached canUndo state
  bool get canUndo => _cachedCanUndo;

  /// Returns the cached canRedo state
  bool get canRedo => _cachedCanRedo;

  /// Returns the cached selection anchor position
  int get selectionAnchor => _cachedSelectionAnchor;

  /// Returns the cached selection active position
  int get selectionActive => _cachedSelectionActive;

  /// Returns true if there is a selection
  bool get hasSelection => _cachedSelectionAnchor != _cachedSelectionActive;

  /// Refreshes the cached canUndo/canRedo state from Rust
  Future<void> refreshUndoRedoState() async {
    _cachedCanUndo = await api.canUndo();
    _cachedCanRedo = await api.canRedo();
  }

  /// Refreshes the selection state from Rust
  Future<void> refreshSelectionState() async {
    _cachedSelectionAnchor = await api.getSelectionAnchor();
    _cachedSelectionActive = await api.getSelectionActive();
  }

  /// Undo the last action
  Future<String> undo() async {
    final result = await api.undo();
    await refreshUndoRedoState();
    return result;
  }

  /// Redo the last undone action
  Future<String> redo() async {
    final result = await api.redo();
    await refreshUndoRedoState();
    return result;
  }

  // ==================== Text Attributes Methods ====================

  /// Gets text attributes at the specified offset
  Future<TextAttributesData> getTextAttributesAt(int offset) async {
    final result = await api.getTextAttributesAt(offset: offset);
    return TextAttributesData.fromString(result);
  }

  /// Gets text attributes at the current selection
  Future<TextAttributesData> getSelectionAttributes() async {
    final start = _cachedSelectionAnchor < _cachedSelectionActive 
        ? _cachedSelectionAnchor 
        : _cachedSelectionActive;
    return getTextAttributesAt(start);
  }

  /// Applies text attributes to the current selection
  Future<String> applyTextAttributes(TextAttributesData attrs) async {
    final start = _cachedSelectionAnchor < _cachedSelectionActive 
        ? _cachedSelectionAnchor 
        : _cachedSelectionActive;
    final end = _cachedSelectionAnchor > _cachedSelectionActive 
        ? _cachedSelectionAnchor 
        : _cachedSelectionActive;
    
    return api.applyTextAttributes(
      start: start,
      end: end,
      attributesJson: attrs.toJson(),
    );
  }

  /// Removes text attributes from the current selection
  Future<String> removeTextAttributes() async {
    final start = _cachedSelectionAnchor < _cachedSelectionActive 
        ? _cachedSelectionAnchor 
        : _cachedSelectionActive;
    final end = _cachedSelectionAnchor > _cachedSelectionActive 
        ? _cachedSelectionAnchor 
        : _cachedSelectionActive;
    
    return api.removeTextAttributes(start: start, end: end);
  }

  /// Applies bold to the current selection
  Future<String> applyBold() async {
    final attrs = TextAttributesData(bold: true);
    return applyTextAttributes(attrs);
  }

  /// Removes bold from the current selection
  Future<String> removeBold() async {
    return removeTextAttributes();
  }

  /// Toggles bold on the current selection
  Future<String> toggleBold() async {
    final currentAttrs = await getSelectionAttributes();
    if (currentAttrs.bold == true) {
      return removeTextAttributes();
    } else {
      return applyBold();
    }
  }

  /// Applies italic to the current selection
  Future<String> applyItalic() async {
    final attrs = TextAttributesData(italic: true);
    return applyTextAttributes(attrs);
  }

  /// Removes italic from the current selection
  Future<String> removeItalic() async {
    return removeTextAttributes();
  }

  /// Toggles italic on the current selection
  Future<String> toggleItalic() async {
    final currentAttrs = await getSelectionAttributes();
    if (currentAttrs.italic == true) {
      return removeTextAttributes();
    } else {
      return applyItalic();
    }
  }

  /// Applies underline to the current selection
  Future<String> applyUnderline() async {
    final attrs = TextAttributesData(underline: true);
    return applyTextAttributes(attrs);
  }

  /// Removes underline from the current selection
  Future<String> removeUnderline() async {
    return removeTextAttributes();
  }

  /// Toggles underline on the current selection
  Future<String> toggleUnderline() async {
    final currentAttrs = await getSelectionAttributes();
    if (currentAttrs.underline == true) {
      return removeTextAttributes();
    } else {
      return applyUnderline();
    }
  }

  /// Sets the font size for the current selection
  Future<String> setFontSize(int size) async {
    final attrs = TextAttributesData(fontSize: size);
    return applyTextAttributes(attrs);
  }

  /// Sets the font family for the current selection
  Future<String> setFontFamily(String family) async {
    final attrs = TextAttributesData(fontFamily: family);
    return applyTextAttributes(attrs);
  }

  /// Sets the text color (foreground) for the current selection
  Future<String> setTextColor(String color) async {
    // Validate hex color format
    if (!color.startsWith('#') || (color.length != 7 && color.length != 9)) {
      throw ArgumentError('Color must be in hex format like #FF0000');
    }
    final attrs = TextAttributesData(foreground: color);
    return applyTextAttributes(attrs);
  }

  /// Sets the background color for the current selection
  Future<String> setBackgroundColor(String color) async {
    // Validate hex color format
    if (!color.startsWith('#') || (color.length != 7 && color.length != 9)) {
      throw ArgumentError('Color must be in hex format like #FF0000');
    }
    final attrs = TextAttributesData(background: color);
    return applyTextAttributes(attrs);
  }

  /// Gets all text with their attributes as JSON
  Future<String> getTextWithAttributes() async {
    return api.getTextWithAttributes();
  }

  /// Layouts the current document state and returns JSON layout information
  Future<String> layoutCurrentDocument(double width) async {
    return api.layoutCurrentDocument(width: width);
  }
}
