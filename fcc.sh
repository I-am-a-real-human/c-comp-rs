#!/usr/bin/bash



print_help() {
    echo "Compiler driver script"
    echo "Usage: $0 [args] <source.c>"
    echo ""
    echo "Optional args:"
    echo "  -l | --lex       Runs lexer and stops"
    echo "  -p | --parse     Runs lexer and parser and stops"
    echo "  -c | --codegen   Runs lexer, parser, assembly generation,"
    echo "                     but stops before code emission"
    echo "  -S               Spits out assembly file <source.s>"
}

CC_BINARY='cc_bin'

# initialise flags
lexer_flag=false
parse_flag=false
codeg_flag=false
ass_flag=false

POSITIONAL=""

while [[ $# -gt 0 ]]; do
    case "$1" in
    	-l|--lex) lexer_flag=true; shift;;
    	-p|--parse) parse_flag=true; shift;;
	-c|--codegen) codeg_flag=true; shift;;
	-S) ass_flag=true; shift;;
	-h|--help) print_help; exit 0 ;;
    	-*) echo "Unknown option '$1'"
	    print_help
	    exit 1
	    ;;
	*)
	    if [[ -z "$POSITIONAL" ]]; then
		POSITIONAL="$1"
	    else
	    	echo "Unexpected argument '$1'"
		print_help
		exit 1
	    fi
	    shift
	    ;;
    esac
done

# validate source file
if [[ -z "$POSITIONAL" ]]; then
  echo "Error: missing file argument."
  show_help
  exit 1
fi

file_path="$POSITIONAL"

if [[ ! -f "$file_path" ]]; then
  echo "Error: file '$file_path' does not exist."
  exit 1
fi

source_file="$POSITIONAL"


# build up combination of flags
if [[ "$lexer_flag" == true ]]; then
    parse_flag=false
    codeg_flag=false
elif [[ "$parse_flag" == true ]]; then
    codeg_flag=false
fi

# start building flags to send to compiler
compiler_args=()

# only one of the stages can be specified
if "$lexer_flag" == true ]]; then
    compiler_args+=('--lex')
elif "$parse_flag" == true ]]; then
    compiler_args+=('--parse')
elif "$codeg_flag" == true ]]; then
    compiler_args+=('--codegen')
fi

if "$ass_flag" true ]]; then
    compiler_args+=('-S')
fi

# run preprocessor
echo "Running preprocessor"
# TODO remove echo and capture return code
gcc -E -P "$POSITIONAL" -o "${source_file%.*}.i"

# check return value of GCC
ret_preproc=$?
if [[ $ret_preproc -ne 0 ]]; then
    echo "Preprocessing failed with exit code $ret_preproc"
    exit $ret_preproc
fi

echo "Compiling..."

"$CC_BINARY" "${compiler_args[@]}"
ret_compile=$?
if [[ $ret_compile -ne 0 ]]; then
    echo "Compilation failed with exit code $ret_compile"
    exit $ret_compile
fi

echo "Assembling..."

gcc "${source_file%.*}.s" -o "${source_file%.*}"

ret_assemble=$?
if [[ $ret_assemble -ne 0 ]]; then
    echo "Assembly failed with exit code $ret_assemble"
    exit $ret_assemble
fi
