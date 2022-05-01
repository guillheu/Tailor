# EULA-protected
This example shows how to use custom request headers to ensure the NFT's EULA was accepted.<br>
In case the header is missing, will display a bad angry picture (which for production could contain your EULA). If the header is present, will display the right image.<br>
This implementation of this use case is far from ideal, and should be addressed when we [implement query parameter dynamic metadata fields](/README.md#roadmap).