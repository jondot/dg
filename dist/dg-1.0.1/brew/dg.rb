class Dg < Formula
  desc "desc"
  homepage "http://github.com/jondot/dg"
  url "https://github.com/jondot/dg/releases/download/v1.0.1/dg-x86_64-macos.tar.xz"
  version "1.0.1"
  sha256 "6f8476dbdeaa5e1c27ba8b7be449de728bb5017f7a8f16deae1f0e0f970a80a4"

  def install
    bin.install "dg"
  end
end
