class Apibrowser < Formula
  desc "Terminal UI for browsing FRC event data from The Blue Alliance"
  homepage "https://github.com/J-kistner/ApiBrowser"
  url "https://github.com/J-kistner/ApiBrowser/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "YOUR_SHA256_HERE"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    assert_match "apibrowser", shell_output("#{bin}/apibrowser --help")
  end
end
