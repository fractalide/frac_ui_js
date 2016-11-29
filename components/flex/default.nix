{ subnet, contracts, components }:

subnet {
  src = ./.;
  subnet = with contracts; with components; ''
   input => input div(${tag}) output => output
   places => places orderer(${orderer}) output -> input div()
   '';
}
