class CocoaWay < Formula
  desc "Universal Wayland Compositor for macOS (Waypipe-Darwin)"
  homepage "https://github.com/J-x-Z/cocoa-way"
  url "https://github.com/J-x-Z/cocoa-way.git", branch: "main"
  version "0.1.0"
  head "https://github.com/J-x-Z/cocoa-way.git", branch: "main"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
    bin.install "run_waypipe.sh"
  end

  test do
    # Verify the binary exists and runs help/version
    assert_match "cocoa-way", shell_output("#{bin}/cocoa-way --help 2>&1", 1)
  end
end
