#!/bin/bash
SCRIPT_PATH=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

TEMPLATE=$SCRIPT_PATH/template.html
CONTENT_PLACEHOLDER="{{ CONTENT }}"

for file in $(find "$SCRIPT_PATH/templates" -name "*.html");
do
  awk -v placeholder="$CONTENT_PLACEHOLDER" -v fp="$file" 'BEGIN{ 
    while ((getline line < fp) > 0) {
      fill = (NR==1 ? line : fill ORS line)
    }
  }
  {
    gsub(placeholder,fill); print
  }' $TEMPLATE > "$SCRIPT_PATH/$(basename $file)"
done
