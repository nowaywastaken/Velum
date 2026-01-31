import 'package:flutter/material.dart';
import 'velum_core_wrapper.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Velum - Word 1:1 Replica',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.blue),
        useMaterial3: true,
      ),
      home: const MyHomePage(title: 'Velum Word Processor'),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({super.key, required this.title});

  final String title;

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  final TextEditingController _controller = TextEditingController();
  bool _isUpdatingFromCore = false;

  @override
  void initState() {
    super.initState();
    _initDocument();
  }

  Future<void> _initDocument() async {
    final core = VelumCoreWrapper();
    final content = await core.api.createEmptyDocument();
    setState(() {
      _isUpdatingFromCore = true;
      _controller.text = content;
      _isUpdatingFromCore = false;
    });
  }

  Future<void> _handleTextChanged(String text) async {
    if (_isUpdatingFromCore) return;

    final core = VelumCoreWrapper();
    // For a basic interactive editor, we'll just sync the whole text for now
    // In a real app, we'd use delta updates with insert_text/delete_text
    // But since we need to use the new API:
    
    // Simple heuristic: if text is longer, it's an insertion, if shorter, deletion.
    // However, for this task, let's just replace the whole content to keep it robust
    // but the instructions say "Connect the editor's changes to the Rust insert_text and delete_text functions".
    
    // To properly use insert/delete, we need to track the previous state.
  }

  // Improved sync logic
  String _previousText = "";

  Future<void> _onChanged(String currentText) async {
    if (_isUpdatingFromCore) {
      _previousText = currentText;
      return;
    }

    final core = VelumCoreWrapper();
    
    if (currentText.length > _previousText.length) {
      // Likely an insertion
      // Find where they differ
      int i = 0;
      while (i < _previousText.length && i < currentText.length && _previousText[i] == currentText[i]) {
        i++;
      }
      int diffLen = currentText.length - _previousText.length;
      String inserted = currentText.substring(i, i + diffLen);
      await core.api.insertText(offset: i, newText: inserted);
    } else if (currentText.length < _previousText.length) {
      // Likely a deletion
      int i = 0;
      while (i < currentText.length && i < _previousText.length && currentText[i] == _previousText[i]) {
        i++;
      }
      int diffLen = _previousText.length - currentText.length;
      await core.api.deleteText(offset: i, length: diffLen);
    }

    _previousText = currentText;
  }

  Future<void> _undo() async {
    final core = VelumCoreWrapper();
    final newText = await core.api.undo();
    setState(() {
      _isUpdatingFromCore = true;
      _controller.text = newText;
      _previousText = newText;
      _isUpdatingFromCore = false;
    });
  }

  Future<void> _redo() async {
    final core = VelumCoreWrapper();
    final newText = await core.api.redo();
    setState(() {
      _isUpdatingFromCore = true;
      _controller.text = newText;
      _previousText = newText;
      _isUpdatingFromCore = false;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        backgroundColor: Theme.of(context).colorScheme.inversePrimary,
        title: Text(widget.title),
        actions: [
          IconButton(
            icon: const Icon(Icons.undo),
            onPressed: _undo,
            tooltip: 'Undo',
          ),
          IconButton(
            icon: const Icon(Icons.redo),
            onPressed: _redo,
            tooltip: 'Redo',
          ),
        ],
      ),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          children: [
            Expanded(
              child: Container(
                decoration: BoxDecoration(
                  border: Border.all(color: Colors.grey.shade300),
                  borderRadius: BorderRadius.circular(8),
                ),
                child: TextField(
                  controller: _controller,
                  maxLines: null,
                  expands: true,
                  onChanged: _onChanged,
                  decoration: const InputDecoration(
                    hintText: 'Start typing...',
                    contentPadding: EdgeInsets.all(12),
                    border: InputBorder.none,
                  ),
                  style: const TextStyle(fontFamily: 'Courier', fontSize: 16),
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
