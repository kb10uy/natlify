use clap::Clap;

#[derive(Clap)]
#[clap(author, version)]
pub struct Arguments {
    /// ドメイン名
    #[clap(short, long, default_value = "www.natorisana.love")]
    pub domain: String,

    /// サブコマンド
    #[clap(subcommand)]
    pub sub_command: SubCommand,
}

#[derive(Clap)]
pub enum SubCommand {
    /// ボタンの一覧を表示する
    List(ListCommand),

    /// 音声をダウンロードする
    Get(GetCommand),
}

#[derive(Clap)]
pub struct ListCommand {
    /// 正規表現によるクエリ
    pub query: Option<String>,
}

#[derive(Clap)]
pub struct GetCommand {
    /// ドメインルートからの data-file 属性のパスまでの相対パス
    #[clap(short, long, default_value = "sounds/")]
    pub sounds_base_path: String,

    /// ダウンロードする音声ファイルの拡張子
    #[clap(short, long, default_value = "mp3")]
    pub extension: String,

    /// ダウンロードする音声のリスト
    pub targets: Vec<String>,
}
