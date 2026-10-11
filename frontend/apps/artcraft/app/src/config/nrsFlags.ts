// NRS build switches. Every feature stays in the code; these two groups are
// off unless the build is made with the variable set to "1".
//
// VITE_NRS_ENABLE_WATERMARK_REMOVERS: the image and video watermark removers.
//   Removing a watermark can strip rights or AI-provenance marks from someone
//   else's work, so they stay hidden unless Justin switches them on.
// The consumer sign-in providers (Sora, Midjourney, Grok and World Labs web
// accounts, which replay a captured browser login) are switched on at run
// time with NRS_ENABLE_CONSUMER_LOGINS=1; see nrs_flags.rs.
export const NRS_ENABLE_WATERMARK_REMOVERS =
  import.meta.env.VITE_NRS_ENABLE_WATERMARK_REMOVERS === "1";

export const NRS_FEATURE_OFF_MESSAGE =
  "This tool is switched off in the NRS Craft build. Ask Justin if you need it.";
