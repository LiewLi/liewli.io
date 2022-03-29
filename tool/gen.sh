set -ex 
TOOL_DIR=`dirname "$0"`
PROJECT_DIR=../$TOOL_DIR
cargo run -- gen --url "$PROJECT_DIR"