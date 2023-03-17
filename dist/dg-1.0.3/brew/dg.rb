class Dg < Formula
  desc "desc"
  homepage "http://github.com/jondot/dg"
  url "https://github.com/jondot/dg/releases/download/v1.0.3/dg-x86_64-macos.tar.xz"
  version "1.0.3"
  sha256 "37e0efc32973a12f4865e7884d733f428cce801567a45555bb8740d548082fce"

  def install
    bin.install "dg"
  end
end
