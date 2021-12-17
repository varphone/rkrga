#!/bin/bash

if [ -z $1 ]; then
    echo "Usage: $0 <binary-file-on-local>"
    exit 1
fi

USER=${TARGET_SSH_USER:=root}
PEER=${TARGET_SSH_HOST:=192.168.66.123}

ENV=LD_LIBRARY_PATH=/usr/local/lib:/usr/lib:/root/lib
LOCAL_FILE=$1
PROG_NAME=$(basename $1)
shift
ARGS=$*
UPLOAD=""
RUN=""

# Upload to the target board via scp
echo ""
echo "===> Uploading ${PROG_NAME} ..."
scp -q ${LOCAL_FILE} ${USER}@${PEER}:/tmp/${PROG_NAME}
CODE=$?

# Run on the target board via ssh if upload okay
if [ ${CODE} -eq 0 ] ; then
    echo "===> Running ${PROG_NAME} ..."
    ssh ${USER}@${PEER} "source /etc/profile; chmod a+x /tmp/${PROG_NAME}; export ${ENV}; /tmp/${PROG_NAME} ${ARGS}; rm -f /tmp/${PROG_NAME}"
    CODE=$?
fi


echo "===< Exited from ${PROG_NAME} = ${CODE}"
echo ""

exit ${CODE}
