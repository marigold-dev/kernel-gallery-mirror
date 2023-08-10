import { blake2bHex } from 'blakejs';
import { RollupClient, Signer } from './rollup';
import { Content, Tweet } from './tweet';
import { TezosToolkit } from '@taquito/taquito';
import { SmartRollupAddMessagesOperation } from '@taquito/taquito/dist/types/operations/smart-rollup-add-messages-operation';
import { COMMITMENT_INTERVAL, BLOCK_TIME, CEMENTED_PERIOD } from '../config';

class Tzwitter {
  private signer: Signer;
  private rollupClient: RollupClient;
  private magicByte: string;

  constructor({
    tezos,
    signer,
    rollupUrl,
    magicByte,
  }: {
    tezos: TezosToolkit;
    signer: Signer;
    rollupUrl: string;
    magicByte?: string;
  }) {
    this.signer = signer;
    this.rollupClient = new RollupClient({ tezos, rollupUrl });
    this.magicByte = '74' || magicByte;
  }

  /**
   * Post a tweet to the rollup
   * @param tweet
   * @returns
   */
  async postTweet(tweet: string): Promise<SmartRollupAddMessagesOperation> {
    const publicKeyHash = await this.signer.publicKeyHash();
    // Compute the next nonce of the user
    const nonceBytes = await this.rollupClient.getState(
      `/accounts/${publicKeyHash}/nonce`,
    );
    const nonce = Number.parseInt(nonceBytes || '00000000', 16) + 1;

    // Hash the payload to sign it later
    const strNonce = nonce.toString(16).padStart(8, '0').toUpperCase();
    const publicKey = await this.signer.publicKey();
    const toHash = `${strNonce}${publicKeyHash}${tweet}`;
    console.log("Data to hash: ", toHash);
    const hash = blake2bHex(toHash, undefined, 32);

    // Sign the payload
    const { prefixSig } = await this.signer.sign(hash);
    // Construct the request
    const request = {
      pkey: {
        Ed25519: publicKey,
      },
      signature: {
        Ed25519: prefixSig,
      },
      inner: {
        nonce: nonce,
        content: {
          PostTweet: {
            author: {
              Tz1: publicKeyHash,
            },
            content: tweet,
          },
        },
      },
    };
    const strRequest = JSON.stringify(request);
    const payload = Buffer.from(strRequest).toString('hex');
    console.log("Hex payload: ", payload);
    // Add the magic byte and send the payload
    return this.rollupClient.send(this.magicByte + payload);
  }

  async handleLoadedImage(hexaString: string) {

    console.log("Posting Hexa image to DAC: ", hexaString);
    const preimageUrl = "https://dac-ghost-coordinator.gcp.marigold.dev/v0/preimage";
    const preimageResponse = await fetch(preimageUrl, {
      method: "POST", // or 'PUT'
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(hexaString),
    })

    const root_hash = await preimageResponse.json();
    // After quick calculation
    // With the actual resources of the DAC
    // We need to wait around 30s/1MB of data
    // Before retrieving the certificate
    await new Promise(resolve => setTimeout(resolve, hexaString.length / 2_000_000 * 30000));

    console.log("Successfully posted image, root_hash is: ", root_hash);
    console.log("Retrieving serialized_certificate from root_hash: ", root_hash);

    const serializedCertificateUrl = "https://dac-ghost-coordinator.gcp.marigold.dev/v0/serialized_certificates/" + root_hash;
    const serializedCertificateResponse = await fetch(serializedCertificateUrl);
    const serializedCertificate = await serializedCertificateResponse.json();

    console.log("Successfully retrieved serialized_certificate: ", serializedCertificate);

    return serializedCertificate;

  };

  /**
   * Post a tweet image to the rollup
   * @param tweet
   * @returns
   */
  async postImage(hexaString: string): Promise<SmartRollupAddMessagesOperation> {

    const serializedCertificate = this.handleLoadedImage(hexaString);

    const publicKeyHash = await this.signer.publicKeyHash();
    // Compute the next nonce of the user
    const nonceBytes = await this.rollupClient.getState(
      `/accounts/${publicKeyHash}/nonce`,
    );
    const nonce = Number.parseInt(nonceBytes || '00000000', 16) + 1;

    // Hash the payload to sign it later
    const strNonce = nonce.toString(16).padStart(8, '0').toUpperCase();
    const publicKey = await this.signer.publicKey();
    const toHash = `${strNonce}${publicKeyHash}${serializedCertificate}`;
    console.log("Data to hash: ", toHash);
    const hash = blake2bHex(toHash, undefined, 32);

    // sign the payload
    const { prefixSig } = await this.signer.sign(hash);

    const request = {
      pkey: {
        Ed25519: publicKey,
      },
      signature: {
        Ed25519: prefixSig,
      },
      inner: {
        nonce: nonce,
        content: {
          PostImage: {
            author: {
              Tz1: publicKeyHash,
            },
            content: serializedCertificate,
          },
        },
      },
    };

    const strRequest = JSON.stringify(request);
    const payload = Buffer.from(strRequest).toString('hex');
    console.log("Hex payload: ", payload);
    // Add the magic byte and send the payload
    return this.rollupClient.send(this.magicByte + payload);

  }


  /**
   * Retrieve a tweet of a given id
   * @param tweetId the id of the tweet
   * @returns the Tweet as a promise
   */
  async getTweet(tweetId: number): Promise<Tweet> {
    const publicKeyHash = await this.signer.publicKeyHash();

    const authorPath = `/tweets/${tweetId}/author`;
    const textContentPath = `/tweets/${tweetId}/content_text`;
    const imgContentPath = `/tweets/${tweetId}/content_img`;
    const likesPath = `/tweets/${tweetId}/likes`;
    const isLikedPath = `/accounts/${publicKeyHash}/likes/${tweetId}`;
    const collectedBlockPath = `/tweets/${tweetId}/collected_level`;

    const authorBytes = await this.rollupClient.getState(authorPath);
    const textContentBytes = await this.rollupClient.getState(textContentPath);
    const imgContentBytes = await this.rollupClient.getState(imgContentPath);
    const likesBytes = await this.rollupClient.getState(likesPath);
    const isLiked = await this.rollupClient.getState(isLikedPath);
    const collectedBlockBytes = await this.rollupClient.getState(
      collectedBlockPath,
    );
    let content: Content;
    if (textContentBytes) {
      const data = Buffer.from(textContentBytes, 'hex').toString('utf-8');
      console.log("Received text from rollup: ", data);
      content = { type: "text", data };
    } else {
      const data = Buffer.from(imgContentBytes, "hex").toString();
      console.log("Received base64 image from rollup: ", data);
      content = { type: "image", data };
    }

    const author = Buffer.from(authorBytes, 'hex').toString('utf-8');
    const likes = Number('0x' + likesBytes);

    // Let's estimate a mint date
    if (collectedBlockBytes) {
      const now = new Date();

      const collectedLevel = Number.parseInt(collectedBlockBytes, 16);

      const currentBlockLevel = await this.rollupClient.tezosLevel();
      const delta = currentBlockLevel - collectedLevel;
      const deltaMs = delta * BLOCK_TIME * 1000;

      /// This commitment duration represent the worst scenario
      /// Let's say the rollup has commit, and your tweet has been collected just after
      // You have to wait 40 blocks to have your transaction included in the rollup commit
      // And then you have to wait 40 other blocks (or 2 weeks on mainnet) for your commit to be cemented
      // If you want to improve this code, you have to know the block of the last commitment: http://localhost:8932/global/last_stored_commitment
      const commitmentDuration = COMMITMENT_INTERVAL * BLOCK_TIME * 1000;
      const cementedDuration = CEMENTED_PERIOD * BLOCK_TIME * 1000;

      const mintableDate = new Date(
        now.getTime() - deltaMs + commitmentDuration + cementedDuration,
      );
      const collected = {
        level: collectedLevel,
        mintableDate: mintableDate,
      };
      return { id: tweetId, author, content, likes, isLiked, collected };
    }
    return { id: tweetId, author, content, likes, isLiked };
  }

  /**
   * Retrieves all the tweets
   * @returns
   */
  async getTweets(): Promise<Array<number>> {
    const path = '/tweets';
    const ids = await this.rollupClient.getSubkeys(path);
    return ids
      .map((id: string) => Number.parseInt(id))
      .sort()
      .reverse();
  }

  /**
   * Get tweets owned by the given public key
   * @param publicKeyHash
   * @returns the id list of owned tweets
   */
  async getOwnedTweets(publicKeyHash: string): Promise<Array<number>> {
    const path = publicKeyHash
      ? `/accounts/${publicKeyHash}/tweets/owned`
      : '/tweets';
    const ids = await this.rollupClient.getSubkeys(path);
    return ids
      .map((id: string) => Number.parseInt(id))
      .sort()
      .reverse();
  }

  /**
   * Get tweets written by the given public key
   * @param publicKeyHash
   * @returns the id list of written tweets
   */
  async getWrittenTweets(publicKeyHash: string): Promise<Array<number>> {
    const path = publicKeyHash
      ? `/accounts/${publicKeyHash}/tweets/written`
      : '/tweets';
    const ids = await this.rollupClient.getSubkeys(path);
    return ids
      .map((id: string) => Number.parseInt(id))
      .sort()
      .reverse();
  }

  /**
   * Like a tweet, it will increment by one the number like of this tweet
   * @param tweetId the id of the tweet to like
   * @returns the hash of the operation
   */
  async like(tweetId: number): Promise<SmartRollupAddMessagesOperation> {
    const publicKeyHash = await this.signer.publicKeyHash();
    // Compute the next nonce of the user
    const nonceBytes = await this.rollupClient.getState(
      `/accounts/${publicKeyHash}/nonce`,
    );
    const nonce = Number.parseInt(nonceBytes || '00000000', 16) + 1;

    // Hash the payload to sign it later
    const strNonce = nonce.toString(16).padStart(8, '0').toUpperCase();
    const publicKey = await this.signer.publicKey();
    const toHash = `${strNonce}${tweetId}`;
    const hash = blake2bHex(toHash, undefined, 32);

    // Sign the payload
    const { prefixSig } = await this.signer.sign(hash);

    // Construct the request
    const request = {
      pkey: {
        Ed25519: publicKey,
      },
      signature: {
        Ed25519: prefixSig,
      },
      inner: {
        nonce: nonce,
        content: {
          LikeTweet: tweetId,
        },
      },
    };
    const strRequest = JSON.stringify(request);
    const payload = Buffer.from(strRequest).toString('hex');
    return this.rollupClient.send(this.magicByte + payload);
  }

  async transferTweet(
    tweetId: number,
    destination: string,
  ): Promise<SmartRollupAddMessagesOperation> {
    const publicKeyHash = await this.signer.publicKeyHash();
    // Compute the next nonce of the user
    const nonceBytes = await this.rollupClient.getState(
      `/accounts/${publicKeyHash}/nonce`,
    );
    const nonce = Number.parseInt(nonceBytes || '00000000', 16) + 1;

    // Hash the payload to sign it later
    const strNonce = nonce.toString(16).padStart(8, '0').toUpperCase();
    const publicKey = await this.signer.publicKey();
    const toHash = `${strNonce}${destination}${tweetId}`;
    const hash = blake2bHex(toHash, undefined, 32);

    // Sign the payload
    const { prefixSig } = await this.signer.sign(hash);
    // Construct the request
    const request = {
      pkey: {
        Ed25519: publicKey,
      },
      signature: {
        Ed25519: prefixSig,
      },
      inner: {
        nonce: nonce,
        content: {
          Transfer: {
            destination: {
              Tz1: destination,
            },
            tweet_id: tweetId,
          },
        },
      },
    };
    const strRequest = JSON.stringify(request);
    const payload = Buffer.from(strRequest).toString('hex');
    console.log(payload);
    // Add the magic byte and send the payload
    return this.rollupClient.send(this.magicByte + payload);
  }

  /**
   * Collect a tweet
   * Then the user has to wait 2 weeks to mint in L1
   * @param tweetId the id of the tweet
   */
  async collect(tweetId: number): Promise<SmartRollupAddMessagesOperation> {
    const publicKeyHash = await this.signer.publicKeyHash();
    // Compute the next nonce of the user
    const nonceBytes = await this.rollupClient.getState(
      `/accounts/${publicKeyHash}/nonce`,
    );
    const nonce = Number.parseInt(nonceBytes || '00000000', 16) + 1;

    // Hash the payload to sign it later
    const strNonce = nonce.toString(16).padStart(8, '0').toUpperCase();
    const publicKey = await this.signer.publicKey();
    const toHash = `${strNonce}${tweetId}`;
    const hash = blake2bHex(toHash, undefined, 32);

    // Sign the payload
    const { prefixSig } = await this.signer.sign(hash);

    // Construct the request
    const request = {
      pkey: {
        Ed25519: publicKey,
      },
      signature: {
        Ed25519: prefixSig,
      },
      inner: {
        nonce: nonce,
        content: {
          Collect: tweetId,
        },
      },
    };
    const strRequest = JSON.stringify(request);
    const payload = Buffer.from(strRequest).toString('hex');
    console.log(this.magicByte + payload);
    return this.rollupClient.send(this.magicByte + payload);
  }

  /**
   * Returns the tweet being collected by the current user
   * @returns the tweet ids of the tweets being collected(frozen on L2, and not minted on L1)
   */
  async getCollectedTweets(publicKeyHash: string): Promise<Array<number>> {
    const path = publicKeyHash
      ? `/accounts/${publicKeyHash}/collecting`
      : '/tweets';
    const ids = await this.rollupClient.getSubkeys(path);
    return ids
      .map((id: string) => Number.parseInt(id))
      .sort()
      .reverse();
  }
}

export { Tzwitter };
