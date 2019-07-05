Pod::Spec.new do |s|
  s.name             = 'StretchKit'
  s.version          = '0.3.2'
  s.summary          = 'Swift bindings for the Stretch layout engine.'
  s.description      = "A high performance & cross-platform layout engine."
  s.homepage         = 'https://vislyhq.github.io/stretch/'
  s.license          = { :type => 'MIT', :file => 'LICENSE' }
  s.author           = { 'Visly Inc.' => 'emil@visly.app' }
  s.source           = { :git => 'https://github.com/vislyhq/stretch.git', :tag => s.version.to_s }
  s.social_media_url = 'https://twitter.com/vislyhq'

  s.swift_version = '4.2'
  s.ios.deployment_target  = '9.3'
  s.source_files = 'bindings/swift/StretchKit/Classes/**/*'
  s.xcconfig = { "ENABLE_BITCODE" => "NO" }

  s.subspec 'StretchCore' do |core|
    core.source_files = 'bindings/swift/StretchKit/Headers/*.h'
    core.vendored_libraries = "bindings/swift/StretchKit/Libraries/libstretch.a"
    core.xcconfig = { "HEADER_SEARCH_PATHS" => "${PODS_ROOT}/StretchCore/Headers" }
    core.preserve_paths = ["bindings/swift/StretchKit/Libraries/libstretch.a", "bindings/swift/StretchKit/Headers/libstretch.h"]
    core.public_header_files = "bindings/swift/StretchKit/Headers/*.h"
    core.requires_arc = false
  end
end
