import { useEffect, useState } from 'react';
import { Tweet } from '../lib/tweet';
import { Tzwitter } from '../lib/tzwitter';
import NumberOfTweets from '../components/NumberOfTweets';
import Feed from '../components/Feed';
import { useBlock } from '../lib/hooks';

type FeedKind = 'owned' | 'written' | 'collecting' | 'all';

interface FeedProperty {
  tzwitter: Tzwitter;
  publicKeyHash?: string;
  onTransfer?: (tweetId: number) => () => void;
  onAuthorClick?: (author: string) => () => void;
  onLike?: (tweetId: number) => () => void;
  feedKind: FeedKind;
  onCollect?: (tweetId: number) => () => void;
}

const FeedContainer = ({
  tzwitter,
  publicKeyHash,
  onTransfer,
  onAuthorClick,
  onLike,
  feedKind,
  onCollect,
}: FeedProperty) => {
  const [tweets, setTweets] = useState<Array<Tweet>>([]);
  const [tweetLikes, setTweetLikes] = useState<Record<string, number>>({});
  const [tweetIsLiked, setTweetIsLiked] = useState<Record<string, boolean>>({});

  useBlock(async () => {
    const getTweets = async (): Promise<Array<number>> => {
      switch (feedKind) {
        case 'owned':
          return publicKeyHash ? tzwitter.getOwnedTweets(publicKeyHash) : [];
        case 'written':
          return publicKeyHash ? tzwitter.getWrittenTweets(publicKeyHash) : [];
        case 'collecting': {
          return publicKeyHash
            ? tzwitter.getCollectedTweets(publicKeyHash)
            : [];
        }
        case 'all':
        default:
          return tzwitter.getTweets();
      }
    };

    const ids = await getTweets();
    const likes = await Promise.all(ids.map((id) => {
      return new Promise(async resolve => {
        const likes = await tzwitter.getLikes(id);
        const isLiked = await tzwitter.getIsLiked(id);
        resolve({ id, likes, isLiked })
      })
    }));

    const likesMap = likes.reduce((acc: Record<string, number>, elt: any) => {
      return { ...acc, [elt.id]: elt.likes }
    }, {})

    const isLikedMap = likes.reduce((acc: Record<string, boolean>, elt: any) => {
      return { ...acc, [elt.id]: elt.isLiked }
    }, {})

    setTweetLikes(likesMap);
    setTweetIsLiked(isLikedMap);
  }, []);


  // Fetch all the tweets
  useBlock(() => {
    const getTweets = async (): Promise<Array<number>> => {
      switch (feedKind) {
        case 'owned':
          return publicKeyHash ? tzwitter.getOwnedTweets(publicKeyHash) : [];
        case 'written':
          return publicKeyHash ? tzwitter.getWrittenTweets(publicKeyHash) : [];
        case 'collecting': {
          return publicKeyHash
            ? tzwitter.getCollectedTweets(publicKeyHash)
            : [];
        }
        case 'all':
        default:
          return tzwitter.getTweets();
      }
    };

    const retrieveTweets = async () => {
      const tzwIds = await getTweets();
      const knownTweetIds = tweets.map(tweet => tweet.id);
      const newTweetIds = tzwIds.filter((tzwIds) => {
        return !knownTweetIds.includes(tzwIds)
      });
      const newTweets = await Promise.all(
        newTweetIds.map((id) => {
          return tzwitter.getTweet(id);
        }),
      );

      newTweets.sort((tweetA, tweetB) => tweetB.id - tweetA.id);

      setTweets(oldTweets => {
        if (newTweets.length > 0) {
          const nextTweets = [...oldTweets, ...newTweets];
          nextTweets.sort((tweetA, tweetB) => tweetB.id - tweetA.id);
          return nextTweets;
        } else {
          return oldTweets
        }
      });
    };
    retrieveTweets();
  }, [feedKind, publicKeyHash, tzwitter, tweets]);

  return (
    <>
      <NumberOfTweets number={tweets.length} />
      <Feed
        tweets={tweets}
        tweetLikes={tweetLikes}
        tweetIsLiked={tweetIsLiked}
        onLike={onLike}
        onAuthorClick={onAuthorClick}
        onTransfer={onTransfer}
        onCollect={onCollect}
      />
    </>
  );
};

export default FeedContainer;
