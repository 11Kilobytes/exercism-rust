(use-modules (guix ci) (guix channels))
(list
 (channel-with-substitutes-available
  %default-guix-channel
  "https://ci.guix.gnu.org"))
