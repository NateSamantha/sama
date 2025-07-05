Introduction
---

Hello! If you are reading this, you have applied for our [Platform Engineer: Proxy](https://fly.io/jobs/platform-proxy/) position.
One of the most important pieces of software you'll work on is `fly-proxy`, and this hiring project is intended to check whether
this is the type of work you'll enjoy doing.

`fly-proxy` is a Rust program built on Tokio and Hyper. It is responsible for accepting user traffic, load-balancing and proxying them to
appropriate user machines.

This repo contains a _very simplified_ version of `fly-proxy` that only forwards raw TCP. Each process here represents an Anycast "edge" host
in our production environment. There are other types of hosts, but they are omitted here for simplicity.

Take a look around this repo, make sure you grok what it is, what it does, and how you would use it.

Challenge 1: Chain of Proxy
---

We've hinted to this -- in production, not every host has a full view into the entire platform. In this version of `fly-proxy`,
both `edge1` and `edge2` have the same data loaded, and can _technically_ route connections to both apps 1 and 2. Currently though, `edge1`
only accepts connections for app 1, so does `edge2` for app 2. Your warm-up task is to make it so that both edges can accept connections
for both apps.

Then, break this by making sure `edge1` and `edge2` only ever know about services in their own region. Trying to connect to app 2
through `edge1` should now result in an error.

Your challenge now is simply to restore the ability for users to connect to app 2 from `edge1` by having `edge1` forward these connections
through `edge2`, and vice-versa. Please make sure your implementation works for any conceivable apps, not just these 2!

We're not looking for a perfect implementation here. Feel free to leave out the last 20%, but do leave some notes on

- What problems do you see in your implementation?
- How would you improve on it for production use?

Challenge 2: Loading Catalog from Corrosion
---

If you got here, you have probably noticed the json blob located under `assets` named `catalog.json`. In production,
this data is stored in a globally distributed eventually-consistent database called [corrosion](https://github.com/superfly/corrosion).

Your challenge is to set up your own Corrosion cluster (a 2-node one is enough!) and make changes to this version of `fly-proxy` to load data from
that cluster. Please note that you do need to handle data updates, not just an initial load like what is done for the mock data right now.
Please include any scripts and test data you have used in the process.

Like before, we're not looking for a 100% production-ready solution here. Leave some notes on future work that needs to be done for your
solution to be production ready. Are there any pitfalls you can see were someone else to work on your implementation?
