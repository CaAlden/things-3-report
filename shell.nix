with import <nixpkgs> {};
with pkgs.python3Packages;

stdenv.mkDerivation {
  name = "python";

  buildInputs = [
    pip
    python310
    virtualenv
  ];

  shellHook = ''
    SOURCE_DATE_EPOCH=$(date +%s)  # so that we can use python wheels
    virtualenv --no-setuptools venv > /dev/null
    export PATH=$PWD/venv/bin:$PATH > /dev/null
    export PIP_DISABLE_PIP_VERSION_CHECK=1
    pip install -r requirements.txt > /dev/null
  '';
}
