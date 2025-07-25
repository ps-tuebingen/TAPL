#!/bin/bash 

VERBOSE=$1
debug() {
  if [[ $VERBOSE == "-v" ]] || [[ $VERBOSE == "--verbose" ]];
  then 
    echo $1
  fi
}

OUT_DIR="crates/web_bin/src/examples";
OUT_MOD="$OUT_DIR/mod.rs";
EXAMPLES_DIR="examples";
EXAMPLE_DIR_TUPLES=();


echo "use std::collections::HashMap;" > $OUT_MOD;
echo "" >> $OUT_MOD;

while IFS='' read -r -d '' example_dir;
do 
  #skip if dir is .
  if [[ $example_dir == $EXAMPLES_DIR ]];
  then 
    continue;
  fi 

  EXAMPLE_DIR_BASE="${example_dir##*/}";
  EXAMPLE_DIR_BASE_DASH=$(echo $EXAMPLE_DIR_BASE | sed 's/_/-/g');
  ALL_EXAMPLES_NAME="${EXAMPLE_DIR_BASE}_all"
  EXAMPLE_DIR_TUPLE="(\"$EXAMPLE_DIR_BASE_DASH\", ${ALL_EXAMPLES_NAME}())";
  EXAMPLE_DIR_TUPLES+=("${EXAMPLE_DIR_TUPLE}");

  OUT_FILE="$EXAMPLE_DIR_BASE.rs";
  OUT_PATH="$OUT_DIR/$OUT_FILE";
  NUM_EXAMPLES=$(ls $example_dir | wc -l);
  NUM_EXAMPLES_STR=$(printf "%02d" $NUM_EXAMPLES);
  CURRENT_NUM=0;
  EXAMPLE_CONST_NAMES=();
  EXAMPLE_NAMES=();

  debug "Loading examples for $EXAMPLE_DIR_BASE";

  #declar examples mod and use array of all examples
  echo "pub mod $EXAMPLE_DIR_BASE;" >> $OUT_MOD;
  echo "pub use $EXAMPLE_DIR_BASE::$ALL_EXAMPLES_NAME;" >> $OUT_MOD;
  echo "" >> $OUT_MOD;

  #fill example mod with example strings
  > $OUT_PATH;
  while IFS='' read -r -d '' example;
  do
    #skip toml configs
    if [[ $example == *".toml"  || $example == $EXAMPLES_DIR ]];
    then 
      continue;
    fi 

    EXAMPLE_BASE="${example##*/}";
    EXAMPLE_TOML="${example%.*}.toml";
    EXAMPLE_NAME="${EXAMPLE_BASE%.*}";
    EXAMPLE_UPPER=$(echo $EXAMPLE_NAME | tr '[:lower:]' '[:upper:]')

    CURRENT_NUM=$(($CURRENT_NUM+1));
    EXAMPLE_CONST_NAMES+=("$EXAMPLE_UPPER");
    EXAMPLE_NAME=$(grep "name=" $EXAMPLE_TOML | sed 's/name=//g' | sed 's/"//g');
    EXAMPLE_NAMES+=("$EXAMPLE_NAME");
    NUM_STR=$(printf "%02d" $CURRENT_NUM);

    debug "($NUM_STR/$NUM_EXAMPLES_STR) $EXAMPLE_NAME";

    #write the file as include_str into the file
    echo "pub const $EXAMPLE_UPPER: &str = include_str!(\"../../../../$example\");" >> $OUT_PATH;
    echo "" >> $OUT_PATH;

  done < <(find $example_dir -type f -print0);

  #write all examples into an array
  echo "pub fn $ALL_EXAMPLES_NAME() -> Vec<(&'static str,&'static str)> { " >> $OUT_PATH;
  printf "    vec![\n" >> $OUT_PATH;
  NUM_EXAMPLES=$(expr ${#EXAMPLE_CONST_NAMES[*]} - 1);
  for NUM_EXAMPLE in $(seq 0 $NUM_EXAMPLES);
  do
    EXAMPLE_CONST_NAME=${EXAMPLE_CONST_NAMES[$NUM_EXAMPLE]};
    EXAMPLE_NAME=${EXAMPLE_NAMES[$NUM_EXAMPLE]};
    echo "         (\"$EXAMPLE_NAME\",$EXAMPLE_CONST_NAME), " >> $OUT_PATH
  done 
  printf "\n" >> $OUT_PATH;
  printf "    ]\n" >> $OUT_PATH;
  echo "}" >> $OUT_PATH;
done < <(find $EXAMPLES_DIR -maxdepth 1 -type d -print0);

#write all arrays into another array 
echo "pub fn all_examples() -> HashMap<&'static str, Vec<(&'static str,&'static str)>> {" >> $OUT_MOD;
echo "    HashMap::from([" >> $OUT_MOD;
for dir_tuple in "${EXAMPLE_DIR_TUPLES[@]}";
do 
  echo "        $dir_tuple," >> $OUT_MOD;
done 
echo "    ])" >> $OUT_MOD;
echo "}" >> $OUT_MOD;
