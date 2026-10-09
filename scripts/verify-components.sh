#!/usr/bin/env bash
# Scaffolds a throwaway Paper project, installs every component from registry/ with the locally built CLI,
# and compiles the lot against the Paper API together with examples/showcase.
#
# Needs: cargo, a JDK 25 (javac) and Maven. Paper 26.x is built for Java 25, so older JDKs fail here.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
WORK="$(mktemp -d)"
PACKAGE="com.example.showcase"

cleanup() {
    rm -rf "$WORK"
}
trap cleanup EXIT

cargo build --quiet --manifest-path "$ROOT/Cargo.toml"
CRAFTCN="$ROOT/target/debug/craftcn"
export CRAFTCN_REGISTRY="$ROOT/registry"

cp "$ROOT/examples/showcase/pom.xml" "$WORK/pom.xml"
cd "$WORK"

"$CRAFTCN" init --package "$PACKAGE" --minecraft 26.2 --yes

COMPONENTS="$(python3 -c 'import json,sys; print(" ".join(c["name"] for c in json.load(open(sys.argv[1]))["components"]))' "$ROOT/registry/index.json")"
for component in $COMPONENTS; do
    echo "craftcn add $component"
    "$CRAFTCN" add "$component" > /dev/null
done

cp -r "$ROOT/examples/showcase/src/main/java/com/example/showcase" "src/main/java/com/example/"
cp -r "$ROOT/examples/showcase/src/main/resources/." "src/main/resources/"

mvn -B -q compile -Dmaven.compiler.compilerArgument=-Xlint:all

echo "All components compile against Paper 26.2."
