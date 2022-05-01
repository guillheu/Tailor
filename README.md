# Tailor
The Tailor project is a suite of powertools for NFT metadata and front-end creation and management. It currently has 2 components :
* `tailor-server-redis` : a project-agnostic redistributable template-based HTTP server.
* `tailor-cli` : a simple utility that will generate and ([eventually](#roadmap)) publish NFT projects based on `tailor-server-redis`.
* Early overview video: https://www.loom.com/share/3ea0ae0a646e458380492111bf09db1d

## Disclaimer
This project is by no means completed and most its features are still being fleshed out.
Currently only supported on linux. Your mileage may vary on other platforms.

## Requirements
Cargo is required to install `tailor-cli`

## Installation
```shell
cargo install --git https://github.com/guillheu/Tailor.git tailor-cli
```

add `--force` to update

## Getting started
To start a new project called `hello-world`, run :
```shell
tailor-cli init hello-world
```
This will create a new `hello-world` folder with a copy of a functional default example.<br>
Alternatively, you can view other examples by using the `example` subcommand, and then entering a name corresponding to one of the folders [in the examples folder](./examples). For instance :
```shell
tailor-cli example static-pictures
```

In this new project, you will find :
* `tailor-server-redis` : the http server binary. It will listen at address `0.0.0.0` on port `8080`.
* `static` : a folder that will hold all your static content. This can include thumbnails, pre-generated monkey jpegs, logos and icons or even a full build of a react or vue  project.
* `templates` : a folder that **MUST** contain 2 files :
  * `metadata.json.hbs` : a json template containing the list of all the metadata for all your tokens.
  * `nft.html.hbs` : an html template of the dynamic front-end for your NFTs. This will be templatized based on the attributes of your NFT metadatas.

To start the server, simply do :
```shell
cd hello-world
./tailor-server-redis
```
Then to access your metadata (declared in the `templates/metadata.json.hbs` file), navigate to [http://localhost:8080/metadata/0](http://localhost:8080/metadata/0).<br>
To view an NFT's dynamic front-end (declared in the `templates/nft.html.hbs` file), go to [http://localhost:8080/?id=0](http://localhost:8080/?id=0).<br>
To view static content, go to [http://localhost:8080/](http://localhost:8080/) followed by the file path within the `static` folder (e.g [/0.png](http://localhost:8080/0.png) for the default example).

## Usage
`tailor-cli` is a command line tool to dynamically create dynamic NFT environments based on the `tailor-server-redis` binary.<br>
```shell
$ tailor-cli --help
tailor-cli 0.1.0
CLI tool for the development and deployment of static and dynamic NFT metadata and servers

USAGE:
    tailor-cli <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    example    Generate a new server directory from a pre-existing example To then run the
                   server, simply run the tailor-server-redis executable. On UNIX systems, add
                   executable permissions to tailor-server-redis
    help       Print this message or the help of the given subcommand(s)
    init       Initialize a new distributable server directory. To then run the server, simply
                   run the tailor-server-redis executable. On UNIX systems, add executable
                   permissions to tailor-server-redis
    publish    Publish a server NOT YET IMPLEMENTED Eventually will allow to publish metadata
                   and NFTs to either Aleph, IPFS or Arweave
```

## Templating
Templating is done using [Handlebars](https://handlebarsjs.com/) ; more specifically the [Rust Handlebars crate](https://crates.io/crates/handlebars).<br>
It will work slightly differently for the metadata template and for the html template.<br>
<br>

### Metadata
The metadata template has access to the following values :
* `headers`: collection containing all the headers from the client's request (accessible through `{{headers.header-name}}`). This is particcularily useful to dynamically retrieve the hostname of the server (in case we don't know it in advance, for instance when deploying to [Aleph.im](https://aleph.im/)). For instance, we can generate the image field of our metadata like so : `"image":"https://{{headers.host}}/0.png"`.
* `get-timestamp-seconds` : returns the server's local epoch timestamp in seconds. **Handlebars does not support native math operations, which is one of the motivation to migrate to Tera.** 
<br>
More dynamic fields will be added over time. Check the [roadmap](#roadmap) for more details.

### Dynamic HTML front-end
The HTML front-end **does NOT have the same dynamic fields as the metadata.** [This is by design](#why-must-we-reference-a-dynamic-value-through-the-metadatas-attributes-before-it-is-accessible-to-the-dynamic-front-end--why-not-just-make-all-values-directly-accessible-to-both-the-front-end-and-the-metadata)<br>
The HTML front-end has access to all the `attributes` declared in the metadata.<br>
For instance, if this is our metadata file `metadata.json.hbs` :
```json
[
  {
    "tokenID": 0,
    "attributes": [
      {
        "trait_type": "Color",
        "value": "green"
      }
    ],
  },
  {
    "tokenID": 1,
    "attributes": [
      {
        "trait_type": "Color",
        "value": "red"
      }
    ],
  }
]
```
Then the following Dynamic font-end `nft.html.hbs` ...
```html
<h1> I am {{Color}} </h1>
```
... will replace `{{Color}}` with the attribute of `trait_type` "Color" of the requested NFT (based on the token ID given as query parameter `/?id=<tokenID>`).<br>
Note that the metadata attribute names are case-sensitive.<br>
<br>
With this, you can include dynamic fields like `get-timestamp-seconds` in your metadata attributes, and then use them in your front-end.



## FAQ
#### Why must we reference a dynamic value through the metadata's attributes before it is accessible to the dynamic front-end ? Why not just make all values directly accessible to both the front-end and the metadata ?
I considered this question with a lot of care. My conclusion is that any dynamic data used in the front-end of an NFT should be referenced in its attributes. I think having clarity between the metadata and the front-end takes precedence over the few lines of template code and possible attribute clutter it generates. This is a design decision I might reconsider, so do let me know if you disagree.

#### Why does the NFT dynamic front end URL use a query parameter for the token ID instead of a path parameter ?
In order to easily maintain better compatibility with relative links in the dynamic front-end. We might consider working on an alternative if that ever becomes an issue.<br>
We are aware that this *could* affect compatibility with certain marketplaces if they strip query parameters from urls in the metadatas. We will keep our ear to the ground to stay on top of issues like this.

#### Why is X made this way ? It would be so much better some other way !
I treasure any and all feedback, as a newbie Rust developper and NFT maker. If you think I messed up somewhere, chances are I did, and I might be already aware of it.<br>
Please check the [roadmap](#roadmap) for any missing feature, or open an issue !

## Roadmap
* Adding Aleph publishing
* Migrating from Handlebars to Tera which is much more featureful, including native math and comparison helpers.
* Adding config file (for port, address, default blockchain & contract address...)
* Adding more dynamic fields
  * protocol (`http` or `https`)
  * all query parameters
  * request body
  * token owner address
  * ...
* Adding metadata generation powertools
* Migrating timestamp generation from the local machine to a remote trusted source (blockchain node or NTP server)
* Adding more publishing backends (IPFS via Pinata, Arweave...)
