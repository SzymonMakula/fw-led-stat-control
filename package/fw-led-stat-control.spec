Name:        fw-led-stat-control
Version:     0.0.1
Release:    1%{?dist}
Summary:  Display system stats on a Framework LED matrix hardware

License:  MIT
URL:      https://github.com/SzymonMakula/%{name}
Source0:  https://github.com/SzymonMakula/%{name}/archive/v%{version}/%{name}-%{version}.tar.gz

Source1: %{name}-%{version}.cargo-vendor.tar.xz
Source2: config.toml
Source3: %{name}-%{version}.plugins.tar.xz

BuildRequires: cargo-rpm-macros >= 24
BuildRequires: systemd-rpm-macros

%description
Display system stats on a Framework LED matrix hardware

%prep
%setup -q -D -T -b0 -n %{name}-%{version}
%setup -q -D -T -b1 -n %{name}-%{version}

# Copy cargo registry overrides so that it reads from vendor directory
mkdir .cargo
cp %{SOURCE2} .cargo/config.toml

# Copy built-in plugins
mkdir -p release/plugins-dist
tar -xf %{SOURCE3} -C release/plugins-dist

%build
export RUSTC_BOOTSTRAP=1
cargo build %{__cargo_common_opts} --release --frozen

%global debug_package %{nil}

%install

mkdir -p %{buildroot}/%{_libdir}/%{name}
mkdir -p %{buildroot}/%{_bindir}
mkdir -p %{buildroot}/%{_unitdir}

# Move library
cp -r release/plugins-dist/plugins %{buildroot}/%{_libdir}/%{name}
cp templates/config.toml %{buildroot}/%{_libdir}/%{name}
install -Dm755 target/release/fw-led-stat-control %{buildroot}/%{_libdir}/%{name}

# Add symlink to binary
ln -s %{_libdir}/%{name}/%{name} %{buildroot}/%{_bindir}/%{name}

# Copy systemd service file
cp package/fw-led-stat-control.service %{buildroot}/%{_unitdir}/%{name}.service

%post
%systemd_post %{name}.service

%preun
%systemd_preun %{name}.service

%postun
%systemd_postun_with_restart %{name}.service

%files
%{_bindir}/%{name}
%{_libdir}/%{name}
%{_unitdir}/%{name}.service