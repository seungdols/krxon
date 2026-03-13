class Krxon < Formula
  desc "CLI tool for KRX (Korea Exchange) Open API"
  homepage "https://github.com/seungdols/krxon"
  url "https://github.com/seungdols/krxon/archive/refs/tags/v0.1.1.tar.gz"
  sha256 "3f80b98c9ed6a6872aab7ab17a0421aeab74bffa6b92bd1367d34af770325d75"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args(path: ".")
  end

  test do
    assert_match "krxon", shell_output("#{bin}/krxon --version")
  end
end
