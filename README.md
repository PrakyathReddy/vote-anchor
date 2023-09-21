# vote-anchor

In this program, we are going to take some content from the internet - any url or any content. We are going to hash that content, and create a unique account for that content and people are going to be able to upvote or downvote on that content in a completely decentralized way.

We are going to have 3 functions:
1. Initialize
2. Upvote
3. Downvote

To test it out, go to beta.solpg.io
Build -> Deploy
1. hash: (copy any piece of text from the internet, convert to hash using a sha256 converter, then convert that array to bytes using https://github.com/Web3-Builders-Alliance/prakyath-reddy-solana-Q3/blob/main/cluster_1/hash_to_array_of_bytes.ts) [
  113,  26,  79,  53, 129, 148, 112,
  137, 209, 132, 236, 240, 225, 194,
  173, 129, 245, 171,  13, 124,  71,
  191,  46,  57,  78,  78, 176,  61,
  198, 190, 250, 174
]
in sol playground u cannot just paste text, u have to convert it into an array of bytes
2. owner: (my address) 7sCXq3U28U2R9XNtnzfXCXUhLidRdfsgwt6gtFytRL33
3. vote: (from seed)(Seed(1)) vote (+)(bytes)(Seed(2)) [
  113,  26,  79,  53, 129, 148, 112,
  137, 209, 132, 236, 240, 225, 194,
  173, 129, 245, 171,  13, 124,  71,
  191,  46,  57,  78,  78, 176,  61,
  198, 190, 250, 174
] Generate that

To view anchor data, deploy the IDL on solpg like
anchor idl init <Program ID>
