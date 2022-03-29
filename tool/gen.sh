TOOL_DIR=`dirname "$0"`
PROJECT_DIR=../$TOOL_DIR
echo $PROJECT_DIR
cargo run -- gen --url "$PROJECT_DIR"