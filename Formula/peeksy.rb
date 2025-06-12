# Documentation: https://docs.brew.sh/Formula-Cookbook
#                https://rubydoc.brew.sh/Formula
# PLEASE REMOVE ALL GENERATED COMMENTS BEFORE SUBMITTING YOUR PULL REQUEST!
class Peeksy < Formula
  desc "screenshot name automation tool for macos"
  homepage "https://anubhavitis.github.io/peeksy/"
  url "https://github.com/anubhavitis/peeksy/releases/download/v0.1.0/peeksy"
  version "v0.1.0"
  sha256 "a6f917ef8a11533e3ffee816843a267a7046f0aaba814b667cf65efe52ab77a5"
  license ""

  def install
    bin.install "peeksy"
  end

  test do
    system "peeksy", "--version"
  end

  def caveats
    <<~EOS
    🎉 🎉 🎉 🎉 🎉 🎉 

    Welcome to Peeksy!

    🎉 🎉 🎉 🎉 🎉 🎉 

    To get started, run:
    peeksy --help

    Thanks for installing Peeksy!

    If you have any questions or feedback, please feel free to reach out to me at anubhavitis@gmail.com

    Cheers!

    EOS
  end
end
