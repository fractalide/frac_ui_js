{ stdenv, buildFractalideSubnet, upkeepers
  , ui_js
  , ...}:

  buildFractalideSubnet rec {
   src = ./.;
   subnet = ''
   input => input div(${ui_js.tag}) output => output
   places => places orderer(${ui_js.orderer}) output -> input div()
   '';

   meta = with stdenv.lib; {
    description = "Subnet: editor card";
    homepage = https://github.com/fractalide/fractalide/tree/master/components/development/test;
    license = with licenses; [ mpl20 ];
    maintainers = with upkeepers; [ dmichiels sjmackenzie];
  };
}
