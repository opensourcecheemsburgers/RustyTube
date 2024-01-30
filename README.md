[![Donations Received][liberapay-donations-received-url]][liberapay-link]
[![Contributors][contributors-shield]][contributors-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![Forks][forks-shield]][forks-url]
[![AGPL-3.0 License][license-shield]][license-url]



# RustyTube

A desktop Youtube client written in Rust. Built with Leptos and Tauri; designed with Tailwind and DaisyUI.



## Why should I use it?
- 🖥️ Why not? Try it out [here][website-url]. 🖥️
- ✨ WebM support for 4k 60fps. ✨
- 👎 Dislike counts. 👎
- 🦀 Built with Rust. 🦀
- 🦋 Beautiful and modern UI. 🦋
- 🎨 30+ themes, including a custom RustyTube theme based on [Sweet Ambar Blue Dark][sweet-theme-url]. 🎨
- 💩 Not an Electron app. 💩



## Screenshots

<div>
    <img src="/screenshots/rt_video_dracula.png" width=300/>
    <img src="/screenshots/rt_video_rustytube.png" width=300/>
    <img src="/screenshots/rt_video_garden.png" width=300/>
    <img src="/screenshots/rt_video_retro.png" width=300/>
</div>

<br></br>

<div>
    <img src="/screenshots/rt_subs_dracula.png" width=300/>
    <img src="/screenshots/rt_subs_rustytube.png" width=300/>
    <img src="/screenshots/rt_subs_garden.png" width=300/>
    <img src="/screenshots/rt_subs_retro.png" width=300/>
</div>



## How do I use RustyTube?

| Platform         | Link                                                        |
|------------------|-------------------------------------------------------------|
| Web              | [rustytube.rs][website-url]                                 |
| Linux (generic)  | [RustyTube_0.1.1.tar.gz][linux-x86_64-generic-url]          |
| Linux (appimage) | [RustyTube_0.1.1_amd64.AppImage][linux-x86_64-appimage-url] |
| Arch             | [AUR package][aur-package-url]                              |
| Debian           | [RustyTube_0.1.1_amd64.deb][debian-package-url]             |
| Windows (exe)    | [RustyTube_0.1.1_x64-setup.exe][windows-exe-url]            |
| Mac              | [rustytube.rs][website-url]                                 |


### Browser Support

| Browser          | Comment                                                     |
|------------------|-------------------------------------------------------------|
| Firefox          | Works perfectly. Testing is done mainly on Firefox.         |
| Chromium-based   | Works great. Might be some ui/ux issues.                    |
| Webkit-based     | Unsupported and a massive PITA. Maybe it will work... idk.  |



## FAQ

### Windows says that RustyTube is a virus. Is it safe to install?

It's totally safe and not a virus; I'm 10 billion percent sure - trust me bro.



## Common Issues

### Video player has a green screen on Webkit-based browsers.

Change the video format. Use the cog located in the bottom right of the video player.

### No audio?

Change the video format. Use the cog located in the bottom right of the video player.

### Video won't load, changing formats doesn't fix it.

If you are trying to watch a music video, RustyTube currently does not work with VEVO videos.




## What's the recipe?

- [Leptos][leptos-url] - A modern Rust web framework. 
- [Tailwind][tailwind-url] - A CSS framework.
- [DaisyUI][daisyui-url] - A Tailwind component library.
- [Tauri][tauri-url] - A Rust desktop application framework.




## Donations

RustyTube is a free and open-source project with well over *500 hours* in development and testing time.

Support the project and its further development via donations.

### Liberapay:

[![Donate via Liberapay][liberapay-donate-button-url]][liberapay-link]

### Monero (XMR):

<img src="/assets/qr_codes/monero_address.svg" width=144 />

```
42bhoxB2DT125WAxLHHZAm2jUpEWUJBe1HAKtmiZsjW6X2r3z8FBqQFEhZdkywuQrrKfnehhhaJexQjVKpahBooq3zYKNjf
```

### Bitcoin (BTC):

<img src="/assets/qr_codes/bitcoin_address.svg" width=144 />

```
bc1q8r90zc8j8a2rvkq4ds8374pxh3rpccxgnjx5x2
```



## Roadmap

RustyTube is still in early development. There are things missing and stuff that I would like to add.

- Mobile UI and Mobile App (Significant donations/support required for this)
- Desktop Integration with Tauri
- Full SponsorBlock API Support
- Full Playlist Support



## Alternatives

### Desktop

[Freetube][freetube-github-url] - An open source desktop YouTube player built with privacy in mind.

### Mobile

[Newpipe][newpipe-github-url] - A libre lightweight streaming front-end for Android. 

[Libretube][libretube-github-url] - An alternative frontend for YouTube, for Android.

[Clipious][clipious-github-url] - Android client application for invidious, the privacy focused youtube front end.



[website-url]: https://rustytube.rs

[sweet-theme-url]: https://github.com/EliverLara/Sweet/tree/Ambar-Blue

[leptos-url]: https://leptos.dev
[tailwind-url]: https://tailwindcss.com
[daisyui-url]: https://daisyui.com
[tauri-url]: https://tauri.app

[linux-x86_64-generic-url]: https://github.com/opensourcecheemsburgers/RustyTube/releases/download/0.1.1-alpha/RustyTube_0.1.1.tar.gz
[linux-x86_64-appimage-url]: https://github.com/opensourcecheemsburgers/RustyTube/releases/download/0.1.1-alpha/RustyTube_0.1.1_amd64.AppImage
[aur-package-url]: https://aur.archlinux.org/packages/rustytube
[debian-package-url]: https://github.com/opensourcecheemsburgers/RustyTube/releases/download/0.1.1-alpha/RustyTube_0.1.1_amd64.deb
[windows-exe-url]: https://github.com/opensourcecheemsburgers/RustyTube/releases/download/0.1.1-alpha/RustyTube_0.1.1_x64-setup.exe
[windows-msi-url]: https://github.com/opensourcecheemsburgers/RustyTube/releases/download/0.1.1-alpha/RustyTube_0.1.1_x64_en-US.msi

[liberapay-link]: https://liberapay.com/opensourcecheemsburgers/donate
[liberapay-donate-button-url]: https://liberapay.com/assets/widgets/donate.svg
[liberapay-donations-received-url]: https://img.shields.io/liberapay/receives/opensourcecheemsburgers.svg?logo=liberapay&style=for-the-badge

[monero-address-svg-url]: assets/qr_codes/monero_address.svg
[bitcoin-address-svg-url]: assets/qr_codes/bitcoin_address.svg


[contributors-shield]: https://img.shields.io/github/contributors/opensourcecheemsburgers/RustyTube.svg?style=for-the-badge
[contributors-url]: https://github.com/opensourcecheemsburgers/RustyTube/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/opensourcecheemsburgers/RustyTube.svg?style=for-the-badge
[forks-url]: https://github.com/opensourcecheemsburgers/RustyTube/network/members
[stars-shield]: https://img.shields.io/github/stars/opensourcecheemsburgers/RustyTube.svg?style=for-the-badge
[stars-url]: https://github.com/opensourcecheemsburgers/RustyTube/stargazers
[issues-shield]: https://img.shields.io/github/issues/opensourcecheemsburgers/RustyTube.svg?style=for-the-badge
[issues-url]: https://github.com/opensourcecheemsburgers/RustyTube/issues
[license-shield]: https://img.shields.io/github/license/opensourcecheemsburgers/RustyTube.svg?style=for-the-badge
[license-url]: https://github.com/opensourcecheemsburgers/RustyTube/blob/master/LICENSE.txt

[freetube-github-url]: https://github.com/FreeTubeApp/FreeTube
[newpipe-github-url]: https://github.com/TeamNewPipe/NewPipe
[libretube-github-url]: https://github.com/libre-tube/LibreTube
[clipious-github-url]: https://github.com/lamarios/clipious
