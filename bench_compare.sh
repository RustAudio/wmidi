#!/usr/bin/env bash

# checkout_and_bench $CHECKOUT $BENCH_OUT
checkout_and_bench() {
  git checkout $1 && \
    cargo bench -- --noplot --save-baseline $2
}

# previous_commit
previous_commit() {
  git log --no-decorate --skip 1 | head -n 1 | cut -d ' ' -f2
}

# Clone the repository
REMOTE_URL="$(git config --get remote.origin.url)";
cd ${TRAVIS_BUILD_DIR}/.. && \
	git clone ${REMOTE_URL} "${TRAVIS_REPO_SLUG}-bench" && \
	cd "${TRAVIS_REPO_SLUG}-bench" && \

checkout_and_bench ${TRAVIS_COMMIT} after
# Bench the previous commit if on master, or master if on a branch.
if [ ${TRAVIS_BRANCH} = master ]
then
  checkout_and_bench $(previous_commit) before
else
  checkout_and_bench master before
fi

critcmp before after;
