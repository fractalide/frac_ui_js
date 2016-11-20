{ stdenv, buildFractalideSubnet, upkeepers
  , tag
  , orderer
  , ...}:

  buildFractalideSubnet rec {
   src = ./.;
   subnet = ''
   input => input div(${tag}) output => output
   places => places orderer(${orderer}) output -> input div()
   '';

   meta = with stdenv.lib; {
    description = "Subnet: editor card";
    homepage = https://github.com/fractalide/fractalide/tree/master/components/development/test;
    license = with licenses; [ mpl20 ];
    maintainers = with upkeepers; [ dmichiels sjmackenzie];
  };
}
