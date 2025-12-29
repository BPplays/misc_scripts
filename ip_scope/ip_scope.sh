#!/usr/bin/env bash
# Print ff{id}{scope}::/16 entries grouped and titled with scope names from RFC 7346.
# RFC 7346: IPv6 Multicast Address Scopes.
scopes=( "1" "2" "3" "4" "5" "8" "e" )

# associative array mapping scope nibble -> RFC name
declare -A scope_name=(
	["0"]="Reserved"
	["1"]="Interface-Local"
	["2"]="Link-Local"
	["3"]="Realm-Local"
	["4"]="Admin-Local"
	["5"]="Site-Local"
	["6"]="Unassigned"
	["7"]="Unassigned"
	["8"]="Organization-Local"
	["9"]="Unassigned"
	["a"]="Unassigned"
	["b"]="Unassigned"
	["c"]="Unassigned"
	["d"]="Unassigned"
	["e"]="Global"
	["f"]="Reserved"
)

# If arg1 provided use it as the template, otherwise use the default
# Default template uses explicit {id} and {scope} placeholders.
name_template=${1:-'ff{id}{scope}::/16'}

for scope in "${scopes[@]}"; do
	# print a title line using the RFC name
	printf '# Scope %s — %s\n' "$scope" "${scope_name[$scope]}"

	# Determine if template includes id
	if [[ "$name_template" == *"y"* || "$name_template" == *"{id}"* ]]; then
		# iterate over ids
		for id in {0..9} {a..f}; do
			out="$name_template"
			out="${out//\{scope\}/$scope}"
			out="${out//x/$scope}"

			# replace id placeholders
			out="${out//\{id\}/$id}"
			out="${out//y/$id}"

			printf '%s\n' "$out"
		done
	else
		# no id placeholder: just print once per scope
		out="$name_template"
		out="${out//\{scope\}/$scope}"
		out="${out//x/$scope}"
		printf '%s\n' "$out"
	fi

	printf "\n"
done
