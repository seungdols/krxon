class Krxon < Formula
  desc "CLI tool for KRX (Korea Exchange) Open API"
  homepage "https://github.com/seungdols/krxon"
  url "https://github.com/seungdols/krxon/archive/refs/tags/v0.1.2.tar.gz"
  sha256 "15cac693d4e38e4b7a8a8cd06f0fa8129ae68a4113cf684701ea902fb7f7f947"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args(path: ".")
  end

  test do
    assert_match "krxon", shell_output("#{bin}/krxon --version")
  end
end
