#!/bin/bash

if [ -z $1 ]; then
    echo "Usage: $0 <binary-file-on-local>"
    exit 1
fi

USER=${TARGET_SSH_USER:=root}
PSWD=${TARGET_SSH_PSWD:=123456}
PEER=${TARGET_SSH_HOST:=192.168.66.123}

ENV=LD_LIBRARY_PATH=/usr/local/lib:/usr/lib:/root/lib
LOCAL_FILE=$1
PROG_NAME=$(basename $1)
shift
ARGS=$*
UPLOAD=""
RUN=""

if [ -f target-runner.env ] ; then
sshpass -p ${PSWD} scp -q target-runner.env ${USER}@${PEER}:/tmp/target-runner.env
fi

# Upload to the target board via scp
echo ""
echo "===> Uploading ${PROG_NAME} ..."
sshpass -p ${PSWD} scp -q ${LOCAL_FILE} ${USER}@${PEER}:/tmp/${PROG_NAME}
CODE=$?

# Run on the target board via ssh if upload okay
if [ ${CODE} -eq 0 ] ; then
    echo "===> Running ${PROG_NAME} ..."
    cat <<EOF | sshpass -p ${PSWD} ssh ${USER}@${PEER} sh -c 'cat > /tmp/target-runner.sh'
#!/bin/sh
export TARGET_DEBUGGER=${TARGET_DEBUGGER}
source /etc/profile
test -f /tmp/target-runner.env && source /tmp/target-runner.env
chmod a+x /tmp/${PROG_NAME}
export ${ENV}
if [ x"\${TARGET_DEBUGGER}" == x"gdb" ]; then
    gdb --args /tmp/${PROG_NAME} ${ARGS}
else
    /tmp/${PROG_NAME} ${ARGS}
fi
rm -f /tmp/${PROG_NAME}
EOF
    sshpass -p ${PSWD} ssh -t ${USER}@${PEER} sh /tmp/target-runner.sh
    CODE=$?
fi

echo "===< Exited from ${PROG_NAME} = ${CODE}"
echo ""

exit ${CODE}
