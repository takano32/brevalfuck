#!/usr/bin/env zsh

emulate -L zsh
setopt extended_glob

typeset -i nest=0

emit() {
	local line=$1
	local pad=${(l:$((nest * 2)):: :)}
	print -r -- "${pad}${line}"
}

cat <<'INIT'
#!/usr/bin/env zsh

emulate -L zsh

typeset -a cell
typeset -i ptr=1
typeset oct ch

cell=(0)

INIT

while IFS= read -r -u 0 -k 1 ch; do
	case $ch in
		'>')
			emit '# >'
			emit '(( ++ptr ))'
			emit '(( ptr > $#cell )) && cell[ptr]=0'
			print
			;;
		'<')
			emit '# <'
			emit '(( --ptr ))'
			print
			;;
		'+')
			emit '# +'
			emit '(( ++cell[ptr] ))'
			print
			;;
		'-')
			emit '# -'
			emit '(( --cell[ptr] ))'
			print
			;;
		'.')
			emit '# .'
			emit 'printf -v oct "%03o" "$(( cell[ptr] & 255 ))"'
			emit 'printf "\\$oct"'
			print
			;;
		',')
			emit '# ,'
			emit 'if IFS= read -r -u 0 -k 1 ch; then'
			(( ++nest ))
			emit 'printf -v "cell[ptr]" "%d" "'\''$ch"'
			(( --nest ))
			emit 'else'
			(( ++nest ))
			emit 'cell[ptr]=0'
			(( --nest ))
			emit 'fi'
			print
			;;
		'[')
			emit '# ['
			emit 'while (( cell[ptr] )); do'
			(( ++nest ))
			print
			;;
		']')
			if (( nest == 0 )); then
				print -ru2 -- 'bf2zsh.zsh: unmatched ]'
				exit 1
			fi
			(( --nest ))
			emit '# ]'
			emit 'done'
			print
			;;
	esac
done

if (( nest != 0 )); then
	print -ru2 -- 'bf2zsh.zsh: unmatched ['
	exit 1
fi
