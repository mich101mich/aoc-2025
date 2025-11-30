#!/bin/bash -e

cd "$(dirname "$0")"

# Expected format: src/days/day_{day:02}_{part}.rs
latest=$(command ls -1 src/days/day_[0-9][0-9]_[0-9].rs | sort | tail -n 1)
latest="${latest#src/days/day_}"
latest="${latest%.rs}"
day="${latest%_*}"
part="${latest#*_}"

# Expected format: input/day_{day:02}.txt (input is shared between parts)
max_input=$(command ls -1 input/day_[0-9][0-9].txt | sort | tail -n 1)
max_input="${max_input#input/day_}"
max_input="${max_input%.txt}"
if [[ "$day" != "$max_input" ]]; then
    echo "Day mismatch: '${max_input}' vs '${day}'"
    exit 1
fi

day="${day:-0}" # On first day, assume last day was 0
part="${part:-2}" # Imaginary day 0 is finished

if [[ $part == "1" ]]; then
    part="2"
else
    part=1
    day="${day#0}" # strip leading 0, as it would cause octal parsing
    day=$((day + 1))
    day=$(printf "%02d" "$day")
fi
echo "day='$day' part='$part'"

input_file="input/day_${day}.txt"
source_file="src/days/day_${day}_${part}.rs"

cp src/current.rs "$source_file"
cp input/current.txt "$input_file"
sed -i -e "s|include_str!(\"../input/current.txt\")|include_str!(\"../../${input_file}\")|g" "$source_file"

if [[ "$part" == "2" ]]; then

    echo -n "" > input/current.txt

    cat > src/current.rs <<EOF
#[allow(unused_imports)]
use crate::utils::*;

pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/current.txt");
    // let input = "";

    // let mut grid = _grid(input);

    let result = input
        //.lines()
        //.chars()
        //.map(parse)
        //.map(|l| sscanf!(l, "").unwrap())
        //.map(|()|)
        //.filter(|()|)
        //.to_vec()
        //.sum::<isize>()
        //.inspect(|x| pv!(x))
        //.count()
        ;

    result!(result);
}
EOF

fi

git -C src/utils add .
git -C src/utils commit -m "Changes for $(basename "$(dirname "$0")") day ${day} part ${part}" || :

git add .
git commit -m "Added day ${day} part ${part}"

if [[ "$part" == "2" ]]; then
    git -C src/utils push
    git push
fi
