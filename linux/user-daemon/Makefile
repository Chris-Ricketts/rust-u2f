export PWD := $(shell pwd)
export TARGETDIR := ${PWD}/../target/fedora

rpm: softu2f-user-daemon.spec
	rpmbuild --define "_sourcedir ${PWD}" --define "_specdir ${PWD}" --define "_builddir ${PWD}" --define "_srcrpmdir ${TARGETDIR}" --define "_rpmdir ${TARGETDIR}" --define "_buildrootdir ${PWD}/.build" -ba softu2f-user-daemon.spec

.PHONY: rpm
