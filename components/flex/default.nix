{ subnet, contracts, components }:

subnet {
  src = ./.;
  flowscript = with contracts; with components; ''
   input => input div(${tag}) output => output
   places => places orderer(${orderer}) output -> input div()
   '';
}
