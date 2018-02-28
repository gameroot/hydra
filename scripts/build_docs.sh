#!/bin/bash
echo "Clearing target/docs dir";
rm -rf target/docs;
echo "Generating new docs"
cargo rustdoc -- --no-defaults --passes "collapse-docs" --passes "unindent-comments"
