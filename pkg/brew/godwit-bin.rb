class GodwitBin < Formula
  version '0.1.1'
  desc "A hackable yet sane project manager and automation suite."
  homepage "https://github.com/Passeriform/GodWit"

  if OS.mac?
      url "https://github.com/Passeriform/GodWit/releases/download/#{version}/GodWit-#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "974351ca7d00083ba2fad52e2f2539c8ff114119c139420f592507962ab43b75"
  elsif OS.linux?
      url "https://github.com/Passeriform/GodWit/releases/download/#{version}/GodWit-#{version}-x86_64-unknown-linux.tar.gz"
      sha256 "c6bba6d643b1a1f18994683e26d4d2b998b41a7a7360e63cb8ec9db8ffbf793c"
  end

  conflicts_with "godwit"

  def install
    bin.install "godwit"
    man1.install "docs/godwit.1"

    bash_completion.install "completions/godwit.bash"
    zsh_completion.install "completions/godwit.zsh"
  end
end
