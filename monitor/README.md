# serapis dir
```bash
sudo mkdir /etc/serapis
```

## config
config needs to be made in `/etc/serapis/monitor.toml`

```toml
account_key = "$account_key"
agent_key = "$agent_key"
base_url = "http://dev.serapis:8000"
```

## plugins

```bash
#if you have monitor-plugins checked out
ln -s /vagrant/monitor-plugins /etc/serapis/plugins

#or just
mkdir /etc/serapis/plugins
```

### /etc/serapis/plugins

in here put files, they need to be executable and have no extension

plugins need to output valid json

## to run

you'll need [rustup.rs](https://www.rustup.rs/), install it then run

```bash
cd $checkout

rustup update nightly

RUST_LOG=serapis_monitor=debug cargo run --features "clippy short_splay"

#if the above fails, you might need to run and try again
cargo update
```

### echo http server

`plackup -p 8080 -MData::Dumper::Concise -e 'sub {my $env = shift; warn "$env->{REQUEST_METHOD} $env->{REQUEST_URI}\n"; $f = $env->{"psgi.input"}; warn <$f> . "\n\n"; [200, [], ["ok"]]}' --no-default-middleware`
