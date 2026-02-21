#!/bin/sh
cd $(dirname $0)

ITERATIONS=25

write_module() {
    printf "module AiBob::CalMod$2 {\n" >> $1
    printf "    fun fun_fun(a: u8) {\n" >> $1
    printf "        assert!(a == 0, 0);\n" >> $1
    printf "    }\n" >> $1
    printf "}\n" >> $1
}

create_move_project() {
    NAME=bundle$1
    smove new $NAME
    TOML=$NAME/Move.toml
    printf 'AiBob = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"' >> $TOML
    for i in $(seq 1 $1)
    do
        FILE=$NAME/sources/Modules.move
        write_module $FILE $i
    done
    sync
    smove bundle -p $NAME
}

sh ./clean.sh
for i in $(seq 1 $ITERATIONS)
do
    create_move_project $i
done
