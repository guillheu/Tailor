# Examples
These are project examples for tailor-server.

## Default example
Simple standard static picture-based NFT series. Does not use any dynamic metadata fields, nor any animations.<br>
The NFT front-ends are only pointed to by the `image` metadata field. This NFT does not use `animation_url`.

## EULA-protected
This example shows how to use custom request headers to ensure the NFT's EULA was accepted.<br>
In case the header is missing, will display a bad angry picture (which for production could contain your EULA). If the header is present, will display the right image.<br>
This implementation of this use case is far from ideal, and should be addressed when we [implement query parameter dynamic metadata fields](/README.md#roadmap).

## Static-images
This example shows how to implement a classic (possibly procedurally generated) image-based NFT series.<br>
The metadata `image` fields simply point to urls to the static files in the `static` folder.

## Threejs-HTML
This example shows how to use your token's attributes directly into a dynamic front-end.

## Time-based-images
This example shows how to use timestamps in your font-end to change a picture.<br>
**/!\\ATTENTION/!\\** the `image` field in the metadata is still **static**. This is because of the limitations of Handlebars which does not let us natively compare values, meaning we cannot dynamically assign an image url based on a timestamp in the metadata. This is some of the motivation behind our decision to [move to Tera for templating in the future](/README.md#roadmap).