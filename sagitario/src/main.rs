use sagitario_debugger::main as debugger_main;
use sagitario_lexer::main as lexer_main;

fn main() {
  lexer_main();
  debugger_main();
}
