class Krxon < Formula
  desc "CLI tool for KRX (Korea Exchange) Open API"
  homepage "https://github.com/seungdols/krxon"
  url "https://github.com/seungdols/krxon/archive/refs/tags/v__VERSION__.tar.gz"
  sha256 "__SOURCE_SHA256__"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args(path: ".")
  end

  test do
    assert_match "krxon", shell_output("#{bin}/krxon --version")
  end
end
