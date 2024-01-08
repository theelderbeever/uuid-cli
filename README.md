# uuid-cli

Because going to UUID generator websites is hard, here is a cli for generating them. Does this exist elsewhere already? I am sure it does but, it was nearly as fast to code it as it was to google it so there we have it.

## Installation

Just gonna have to `cargo install` this one...

`cargo install --git  https://github.com/theelderbeever/uuid-cli.git`

## Usage

The default version is set to `v4`. However, you can pass other versions as you like (those aren't implemented right now).

```console
❯ uuid
c0e0ab24-307b-4f3b-a5e1-dbc7765654e5
❯ uuid v4
468e70b9-3de5-4339-9477-f32f28dc0b88
❯ uuid -n 4
4fe12cf3-012e-4bc0-a9bf-5247635ddb12
fdd47e5f-001f-4b00-a774-5914803898d8
c364703f-ab4c-47b8-b396-244719c201e8
7e39b1b4-e733-4da8-aa22-bfa08c67595c
❯ uuid -n 4 > uuids.txt && cat uuids.txt
20ff9100-5c13-4705-a19d-8caadfee0e12
b3ba33fc-cac9-4200-abf2-b444c44e13d5
b790aedb-522b-43d6-8d22-7c553cee2c29
bd10c52e-2d32-4e34-8f97-c52108b4530c
```