#!/bin/bash
test -d tmp || mkdir tmp
EXE=$(mktemp tmp/ua_bot_XXXXX)
touch $EXE
cat <<EOF > $EXE
#!/bin/bash
exec $@ "\$@"
EOF
chmod +x $EXE
echo -n $EXE
