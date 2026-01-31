import 'package:flutter/material.dart';
import 'velum_core_wrapper.dart';

class VelumToolbar extends StatefulWidget implements PreferredSizeWidget {
  final VoidCallback onUndo;
  final VoidCallback onRedo;
  final VoidCallback onSave;
  final VoidCallback onOpen;
  final bool canUndo;
  final bool canRedo;
  final bool isSaving;
  
  // Text formatting callbacks
  final VoidCallback onToggleBold;
  final VoidCallback onToggleItalic;
  final VoidCallback onToggleUnderline;
  final Function(int) onSetFontSize;
  final Function(String) onSetTextColor;
  final Function(String) onSetBackgroundColor;
  
  // Active state indicators
  final bool isBold;
  final bool isItalic;
  final bool isUnderline;
  final bool hasSelection;
  
  const VelumToolbar({
    super.key,
    required this.onUndo,
    required this.onRedo,
    required this.onSave,
    required this.onOpen,
    this.canUndo = true,
    this.canRedo = true,
    this.isSaving = false,
    required this.onToggleBold,
    required this.onToggleItalic,
    required this.onToggleUnderline,
    required this.onSetFontSize,
    required this.onSetTextColor,
    required this.onSetBackgroundColor,
    this.isBold = false,
    this.isItalic = false,
    this.isUnderline = false,
    this.hasSelection = false,
  });
  
  @override
  Size get preferredSize => const Size.fromHeight(kToolbarHeight);
  
  @override
  State<VelumToolbar> createState() => _VelumToolbarState();
}

class _VelumToolbarState extends State<VelumToolbar> {
  bool _showFontSizeDialog = false;
  bool _showTextColorDialog = false;
  bool _showBgColorDialog = false;

  @override
  Widget build(BuildContext context) {
    return AppBar(
      title: const Text('Velum'),
      actions: [
        _ToolbarButton(
          icon: Icons.undo,
          tooltip: 'Undo (Cmd+Z)',
          onPressed: widget.canUndo ? widget.onUndo : null,
        ),
        _ToolbarButton(
          icon: Icons.redo,
          tooltip: 'Redo (Cmd+Shift+Z)',
          onPressed: widget.canRedo ? widget.onRedo : null,
        ),
        const SizedBox(width: 8),
        _ToolbarButton(
          icon: isSaving ? Icons.save_alt : Icons.save,
          tooltip: 'Save (Cmd+S)',
          onPressed: widget.onSave,
        ),
        _ToolbarButton(
          icon: Icons.folder_open,
          tooltip: 'Open File',
          onPressed: widget.onOpen,
        ),
        const SizedBox(width: 16),
        // Text formatting buttons
        _ToolbarButton(
          icon: Icons.format_bold,
          tooltip: 'Bold (Cmd+B)',
          onPressed: widget.hasSelection ? widget.onToggleBold : null,
          isActive: widget.isBold,
        ),
        _ToolbarButton(
          icon: Icons.format_italic,
          tooltip: 'Italic (Cmd+I)',
          onPressed: widget.hasSelection ? widget.onToggleItalic : null,
          isActive: widget.isItalic,
        ),
        _ToolbarButton(
          icon: Icons.format_underline,
          tooltip: 'Underline (Cmd+U)',
          onPressed: widget.hasSelection ? widget.onToggleUnderline : null,
          isActive: widget.isUnderline,
        ),
        const SizedBox(width: 8),
        // Font size button
        _ToolbarButton(
          icon: Icons.format_size,
          tooltip: 'Font Size',
          onPressed: widget.hasSelection ? _showFontSizeSelector : null,
        ),
        const SizedBox(width: 8),
        // Text color button
        _ToolbarButton(
          icon: Icons.text_format,
          tooltip: 'Text Color',
          onPressed: widget.hasSelection ? _showTextColorSelector : null,
        ),
        // Background color button
        _ToolbarButton(
          icon: Icons.highlight,
          tooltip: 'Background Color',
          onPressed: widget.hasSelection ? _showBgColorSelector : null,
        ),
      ],
    );
  }

  void _showFontSizeSelector() {
    showDialog(
      context: context,
      builder: (context) => FontSizeDialog(
        onFontSizeSelected: (size) {
          widget.onSetFontSize(size);
          Navigator.of(context).pop();
        },
      ),
    );
  }

  void _showTextColorSelector() {
    showDialog(
      context: context,
      builder: (context) => ColorDialog(
        title: 'Text Color',
        onColorSelected: (color) {
          widget.onSetTextColor(color);
          Navigator.of(context).pop();
        },
      ),
    );
  }

  void _showBgColorSelector() {
    showDialog(
      context: context,
      builder: (context) => ColorDialog(
        title: 'Background Color',
        onColorSelected: (color) {
          widget.onSetBackgroundColor(color);
          Navigator.of(context).pop();
        },
      ),
    );
  }
}

class _ToolbarButton extends StatelessWidget {
  final IconData icon;
  final String tooltip;
  final VoidCallback? onPressed;
  final bool isActive;

  const _ToolbarButton({
    required this.icon,
    required this.tooltip,
    required this.onPressed,
    this.isActive = false,
  });
  
  @override
  Widget build(BuildContext context) {
    final isDark = Theme.of(context).brightness == Brightness.dark;
    final activeColor = isDark ? Colors.white : Colors.black87;
    final inactiveColor = Colors.grey[400]!;
    
    return IconButton(
      icon: Icon(icon),
      tooltip: tooltip,
      onPressed: onPressed,
      color: onPressed != null ? (isActive ? Colors.blue : activeColor) : inactiveColor,
      splashRadius: 20,
    );
  }
}

class FontSizeDialog extends StatefulWidget {
  final Function(int) onFontSizeSelected;

  const FontSizeDialog({super.key, required this.onFontSizeSelected});

  @override
  State<FontSizeDialog> createState() => _FontSizeDialogState();
}

class _FontSizeDialogState extends State<FontSizeDialog> {
  int _selectedSize = 16;

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: const Text('Font Size'),
      content: SizedBox(
        width: 200,
        height: 200,
        child: ListView.builder(
          itemCount: 20,
          itemBuilder: (context, index) {
            final size = 8 + index * 2;
            return ListTile(
              title: Text('$size pt'),
              selected: _selectedSize == size,
              selectedColor: Colors.blue,
              onPressed: () {
                setState(() {
                  _selectedSize = size;
                });
              },
            );
          },
        ),
      ),
      actions: [
        TextButton(
          onPressed: () => Navigator.of(context).pop(),
          child: const Text('Cancel'),
        ),
        ElevatedButton(
          onPressed: () => widget.onFontSizeSelected(_selectedSize),
          child: const Text('Apply'),
        ),
      ],
    );
  }
}

class ColorDialog extends StatefulWidget {
  final String title;
  final Function(String) onColorSelected;

  const ColorDialog({
    super.key,
    required this.title,
    required this.onColorSelected,
  });

  @override
  State<ColorDialog> createState() => _ColorDialogState();
}

class _ColorDialogState extends State<ColorDialog> {
  static const List<ColorOption> _colorOptions = [
    ColorOption('#000000', 'Black'),
    ColorOption('#FFFFFF', 'White'),
    ColorOption('#FF0000', 'Red'),
    ColorOption('#00FF00', 'Green'),
    ColorOption('#0000FF', 'Blue'),
    ColorOption('#FFFF00', 'Yellow'),
    ColorOption('#FF00FF', 'Magenta'),
    ColorOption('#00FFFF', 'Cyan'),
    ColorOption('#FFA500', 'Orange'),
    ColorOption('#800080', 'Purple'),
    ColorOption('#008000', 'Dark Green'),
    ColorOption('#000080', 'Dark Blue'),
    ColorOption('#8B0000', 'Dark Red'),
    ColorOption('#FFC0CB', 'Pink'),
    ColorOption('#808080', 'Gray'),
    ColorOption('#D2B48C', 'Tan'),
  ];

  String _selectedColor = '#000000';

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: Text(widget.title),
      content: SizedBox(
        width: 250,
        height: 300,
        child: GridView.builder(
          gridDelegate: const SliverGridDelegateWithFixedCrossAxisCount(
            crossAxisCount: 4,
            crossAxisSpacing: 8,
            mainAxisSpacing: 8,
          ),
          itemCount: _colorOptions.length,
          itemBuilder: (context, index) {
            final option = _colorOptions[index];
            final color = _parseColor(option.hex);
            return InkWell(
              onTap: () {
                setState(() {
                  _selectedColor = option.hex;
                });
              },
              child: Container(
                decoration: BoxDecoration(
                  color: color,
                  border: Border.all(
                    color: _selectedColor == option.hex ? Colors.blue : Colors.grey,
                    width: _selectedColor == option.hex ? 3 : 1,
                  ),
                  borderRadius: BorderRadius.circular(4),
                ),
                child: _selectedColor == option.hex
                    ? const Icon(Icons.check, color: Colors.white)
                    : null,
              ),
            );
          },
        ),
      ),
      actions: [
        TextButton(
          onPressed: () => Navigator.of(context).pop(),
          child: const Text('Cancel'),
        ),
        ElevatedButton(
          onPressed: () => widget.onColorSelected(_selectedColor),
          child: const Text('Apply'),
        ),
      ],
    );
  }

  Color _parseColor(String hex) {
    try {
      final value = int.parse(hex.substring(1), radix: 16);
      return Color(value | 0xFF000000);
    } catch (e) {
      return Colors.black;
    }
  }
}

class ColorOption {
  final String hex;
  final String name;

  const ColorOption(this.hex, this.name);
}
