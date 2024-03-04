import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaSocialMedia } from "../target/types/solana_social_media";

describe("solana_social_media", () => {

  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SolanaSocialMedia as Program<SolanaSocialMedia>
  const provider = anchor.getProvider()

  const users = []

  before(async () => {

    const timestamp = Math.floor(Date.now() / (60 * 60 * 24) / 1000) * 1000
    let postcount = 0
    console.log(timestamp)
    console.log(Buffer.from(timestamp.toString()))

    const {
      LAMPORTS_PER_SOL
    } = anchor.web3

    // deployer
    // poster a
    // poster b
    // comment a
    // comment b
    // 

    { // keypair accounts:
      {


        const account = anchor.web3.Keypair.generate()

        const tx = await provider.connection.requestAirdrop(account.publicKey, 1 * LAMPORTS_PER_SOL)
        const blockhash = await provider.connection.getLatestBlockhash()

        await provider.connection.confirmTransaction({
          ...blockhash,
          signature: tx
        }, 'confirmed')

        users.push({
          keypair: account, type: 'treasury'
        })
      }


      {
        const account = anchor.web3.Keypair.generate()

        const tx = await provider.connection.requestAirdrop(account.publicKey, 10000 * LAMPORTS_PER_SOL)
        const blockhash = await provider.connection.getLatestBlockhash()

        await provider.connection.confirmTransaction({
          ...blockhash,
          signature: tx
        }, 'confirmed')

        users.push({
          keypair: account, type: 'poster_a'
        })
      }
    }


    { // program derived accounts

      {
        const [senddit] = anchor.web3.PublicKey.findProgramAddressSync(
          [Buffer.from('senddit')],
          program.programId
        )

        users.push({
          publicKey: senddit, type: 'senddit'
        })
      }

      {
        const [postStore] = anchor.web3.PublicKey.findProgramAddressSync(
          [Buffer.from(timestamp.toString())],
          program.programId
        )

        users.push({
          publicKey: postStore, type: 'post_store'
        })
      }

      {
        const link = 'shared-link-0.com'
        const postStore = users.find(user => user.type === 'post_store')
        // get post_store.posts data, for now use post count
        const [postLink] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            postStore.publicKey.toBuffer(),
            Buffer.from((++postcount).toString())
          ],
          program.programId
        )

        const [post_pda] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            Buffer.from(link)
          ],
          program.programId
        )

        users.push({
          publicKey: postLink, link, count: postcount, collision: post_pda, type: 'post_link'
        })
      }


    }





  })

  it("Is initialized!", async () => {

    const payer = users.find(user => user.type === 'treasury').keypair
    const senddit = users.find(user => user.type === 'senddit')

    const tx = await program.methods
      .initialize()
      .accounts({
        authority: payer.publicKey,
        senddit: senddit.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId
      })
      .signers([payer])
      .rpc();

    const blockhash = await provider.connection.getLatestBlockhash()
    await provider.connection.confirmTransaction({
      ...blockhash,
      signature: tx
    }, 'confirmed')
  });


  it("Init Post Store!", async () => {

    const payer = users.find(user => user.type === 'poster_a').keypair
    const treasury = users.find(user => user.type === 'treasury').keypair
    const senddit = users.find(user => user.type === 'senddit')
    const postStore = users.find(user => user.type === 'post_store')

    const tx = await program.methods
      .initPostStore()
      .accounts({
        authority: payer.publicKey,
        treasury: treasury.publicKey,
        senddit: senddit.publicKey,
        postStore: postStore.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId
      })
      .signers([payer])
      .rpc();

    const blockhash = await provider.connection.getLatestBlockhash()
    await provider.connection.confirmTransaction({
      ...blockhash,
      signature: tx
    }, 'confirmed')
  });


  it("Post Link!", async () => {

    const payer = users.find(user => user.type === 'poster_a').keypair
    const treasury = users.find(user => user.type === 'treasury').keypair
    const senddit = users.find(user => user.type === 'senddit')
    const postStore = users.find(user => user.type === 'post_store')
    const post = users.find(user => user.type === 'post_link')

    const tx = await program.methods
      .postLink(post.link)
      .accounts({
        authority: payer.publicKey,
        treasury: treasury.publicKey,
        senddit: senddit.publicKey,
        postStore: postStore.publicKey,
        posterWallet: payer.publicKey,
        post: post.publicKey,
        postPda: post.collision,
        systemProgram: anchor.web3.SystemProgram.programId
      })
      .signers([payer])
      .rpc();

    const blockhash = await provider.connection.getLatestBlockhash()
    await provider.connection.confirmTransaction({
      ...blockhash,
      signature: tx
    }, 'confirmed')
  });


  it("Upvote Post!", async () => {

    const senddit = users.find(user => user.type === 'senddit')
    const treasury = users.find(user => user.type === 'treasury').keypair

    // const payer = users.find(user => user.type === 'poster_b').keypair
    const postStore = users.find(user => user.type === 'post_store')
    const posterWallet = users.find(user => user.type === 'poster_a').keypair
    const post = users.find(user => user.type === 'post_link')

    const tx = await program.methods
      .upvotePost(post.count.toString())
      .accounts({
        authority: treasury.publicKey,
        treasury: treasury.publicKey,
        senddit: senddit.publicKey,
        postStore: postStore.publicKey,
        posterWallet: posterWallet.publicKey,
        post: post.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId
      })
      .signers([treasury])
      .rpc();

    const blockhash = await provider.connection.getLatestBlockhash()
    await provider.connection.confirmTransaction({
      ...blockhash,
      signature: tx
    }, 'confirmed')
  });


});

// sessions
// rankings
// rewards
//    -> winning submission
//    -> vote with highest weight
// rewards are available and determined after the ending of each session
// submit info -> link

// reduce spam and ensure that there is an incentive to provide quality and valuble content
// there is a variable cost to sharing content. funds are locked for the duration
// of a given session, and are aviable after the session
// if there consensous that the context has no value then those locked funds are distributed
// evenly to the top 10 content providers that locked up the most capital

// reputation system
// - creates an incentive to participants to use some address, 
//   instead of creating new address to cheat the system. how can the system be cheated, or exploited? 
//  -> content submitters / creators
//  -> voters
//  

// spam detection / filter system
// - contributers can [mark content / make claim of content] as spam
// - contributers must meet a set of conditions to mark content as spam
// - marked content must go through a consensus round
// - consensus round is a 24 hour period
// - consensus members vote on marked content, majority determines the outcome of marked content
// - consensus member must have locked tokens and meet various conditions to vote
// - consensus options [approve claim, reject claim]
// - if content is approved as spam
// - - contributer owner of content that is marked as spam loses funds 
// - - the funds are distributed amoung other contributers, consensus members and top members of leader board of reputation system
// - - 
// - if content is rejected as spam
// - - contributers that marked content as spam have their locked funds deducted
// - - consensus members that voted the minority have their locked funds deducted
// - - contriubters reputation is negatively impacted
// - - consensus members reputation is negatively impacted
// - - reputation system determines how much funds are lost for each member, higher reputation reduces the amount that could be loss
// - - the funds are distributed amoung the other contributers, consensus members and top members of leader board of reputation system



// challenged content:
//  RATIONAL: the why
//    - reduce spam contant, incentivizing to share content that others will find of value.
//    - prevent illegal content from being distributed on system
//    - solacous or comprimising content can be challenged and remove
//    - devisive content can defend their position if challenged, a fair system
//    - system to minimize abuse and everyone plays equally.
//  FEES:
//    - fees are collected for each wager amount, they increase 2x
//  CONDITIONS:
//    - contributor of current session can challenge content, and 2x the minimum base amount or match challenged content locked amount
//    - consensus members that pool together the base minimum amount to challenge content
//    - those that mark content are labled the challenger
//    - those that defend the content are labled the challenged
//    - members can defend a content 
//    - challenged content must be marked on general reason / catagory
//   STRUCTURE:
//    - locked amount -> portion or all is risked based on relative or min RP score -> is the risked locked amount
//    - wager amount  -> all is risked
//   RISK:
//    - challenger [members, contributor]
//    - content contributor risked locked amount table
//        - contributors risk all their locked tokens when reputation score is below 50% of given session or below 10 ponts in RP score
//        - contributors that are top 50% in reputation score of given session risk 1/2 of their locked tokens
//        - contributors that are top 25% in reputation score of given session risk 1/4 of their locked tokens 
//        - contributors that are top 12.5% in reputation score of given session risk 1/8 of their locked tokens
//    - defender
//        - wager a minimum of threshold amount to defend content
//        - 25% of challenger wager amount to collect 6.25% of wager amount
//        - 50% of challenger wager amount to collect 12.5% of wager amount
//        - 100% of challenger wager amount to collect 25% of wager amount
//        - 200% of challenger wager amount to collect 50% of wager amount
//        - 400% of challenger wager amount to collect 75% of wager amount
//        - 800% of challenger wager amount to collect 87.5% of wager amount
//        - remaining wager amount is distributed amoung the majority voting consensus members
//        - 50% of challenger risked locked amount is distributed amoung the majority voting consensu members
//        - 50% of challenger risked locked amount is given to defender
//        - contributor at risked locked amount is also at risk
//    - challenger
//        - must wager 2x the minimum threshold amount to challenge content
//        - 25% of defender wager amount to collect 6.25% of wager amount
//        - 50% of defender wager amount to collect 12.5% of wager amount
//        - 100% of defender wager amount to collect 25% of wager amount
//        - 200% of defender wager amount to collect 50% of wager amount
//        - 400% of defender wager amount to collect 75% of wager amount
//        - 800% of defender wager amount to collect 87.5% of wager amount
//        - remaining wager amount is distributed amoung the majority voting consensus members
//        - 50% of defender risked locked amount is distributed amoung the majority voting consensu members
//        - 50% of defender risked locked amount is given to defender
//        - if challenger is contributor, their at risked locked amount is also at risk,
//        - challenger can revoke their challenge at a small penelty and fee before consensus round starts
//    - reputation score is reduced for losing side
//        - how much should the reputation score be impacted?
//    - illegal content takes more harsher penelties. 
//        - defender at risk to losing all collective locked funds
//        - defender at risk being black listed
//        - challenger at risk losing all locked funds
//        - defender can defend position a second time after consensus, by waging large amount?
//        - defender takes all locked funds of all particpants in vote if won?
//        - this is a complex problem to solve
//   VOTE:
//    - votes cost a small fee to cast a vote or remove a vote
//    - votes are finalized after consensus round.
//    - if votes rapidly change in last 5 minutes of round, changing votes lose 25% of their locked funds
//    -

// consensus member:
//  CONDITION:
//    - any account that is not black listed can be a consensus member
//    - members are elgible to vote once locked tokens that have not been unstaked after a period of 10 days.
//    - members are elgible to collect rewards for locking tokens after 30 day period
//    - members can unlock tokens at any moment / all tokens are payed back
//    - members can unlock any amount?
//    - there is a small cost to locking and unlocking tokens
//    - fees (2)x for every 100 members locking/unlocking tokens in an given 4 hour weighted average period, 1 hour tick      : (members, period) -> not set in stone.
//    - feex (1/2)x if less than 100 members locking/unlocking tokens in a given 4 hour weighted average period, 1 hour tick  : (members, period) -> not set in stone.
//  RISK:
//    - during voting sessions if members are not part of the majority, they lose part or all of their funds
//    - when it involves illegal content the minority lose all their funds, this can be challneged multiple times, and if challenged again members of previous majority lose funds?
//    - portion of those funds are distributed to the majority.
//    - portion of those funds are placed in a pool?
//    - portion of those funds are distributed to contributors of current session?
//    - this incentivies that all votes go in one direction, and if is the right direction, consensus will be near 100%
//    - in cases that their is a real division on consensus what falls in a 40% to 60% distribution, considerations must be taken.
//      -> an algorithm must take into account of such sitations, since this signafies a real division on consensus and not an attempted exploitation of the system.
//  REWARDS:
//    - members earn rewards for being part of the majority of a consensus phase
//    - tokens are locked in a incubation phase for 10 days, just in case the consensus phase is challenged.
//    - after a consecutive of 10 majority votes members are elgible for minting tokens, only members of top 10% can mint tokens
//    - 10% members with the largest locked balance can also mint tokens
//  VOTES:
//    - members can only vote once on a marked content.
//    - members are alloted 1 vote per consensus round
//    - for any and each additional votes, members must add a tiny amount to their locked balance.
//    - the majority determines the state of the content.

// session phases and rounds
// phase 1
//  -> 48 hour peroid submit content for session
// phase 2
//  -> 24 hour period to validate value content, spam content removed
//  -> the amount of spam content found determins the next minimum thresh hold for phase 1
// phase 3
//  -> 72 hour period to vote top content


// submit content
//  INPUTS:
//      -> content [pointer or reference to content]
//      -> catagory
//      -> amount
//  NOTES:
//      fees: 
//        - fees collected for each submission.
//      minimum threshold:
//        - wager amount must meet minimum threshold
//        - minimum threshold amount is determine by reputation score.
//        - if reputation score is 0, the minumum threshold is the base minimum threshold
//        - base minimum threshold is dynamic, it increases if spam content increases, and decreases over time if no spam content is approved per session.
//      reputation score:
//        - reputation score increases 1 point for locked amount that is an exponential of the minimum amount of posted content
//          - -> ex.) minumum = 200 tokens, submitted amount 2x minumum = 400 tokens adds 1 point to reputation
//          - -> ex.) minumum = 200 tokens, submitted amount 4x minumum = 800 tokens adds 2 points to reputation.
//          - -> ex.) minumum = 200 tokens, submitted amount 8x minimum = 1600 tokens adds 3 points to reputation.
//        - the accumlated points will be added to the reputation system when submitted content is approved and finalized.
//          before that, accumulated points are in a staging area and have no impact yet.
//        - the amount of content a contributor can submit per session depends on the reputation score / ranking
//          the higher the reputation score of contributor the more distinct content a contributor can submit per session.
//      risk:
//        - during consensus phase the locked tokens can be slashed and given to members of the system if accepted as spam content
//        - any ilegal content will blacklist the contributors address and will loose all funds
//      rewards: Voting phase
//        - 
//      stake:
//        - tokens remain locked after consensus phase for a set duration and can be unlocked after a maturity date
//        - the amount of tokens locked and duration of locking period is a contributing factor on visibility, discovery of content, and voting



// types of participants
//  -> contributors
//  -> voters
//  -> content submittes -> is this different from contributors? I think should be consider the same
//  -> consensus member


// voting phase, different catagories and sessions
//  - new content
//  - favorite or most popular content
//  - longest locked content
//  - most engaging content
//  - etc..

