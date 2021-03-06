//! 整合性関連の構成要素。

/// 参照系の処理における整合性保証のレベルを表す。
///
/// 整合性指定により、MDS から参照されるオブジェクトが最新か否かに影響を与える。
/// 強整合性は、常に最新のオブジェクトが参照できることを意味する。
/// 弱整合性は、最新ではない、古くなったオブジェクトが参照される可能性があることを意味する。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReadConsistency {
    /// オブジェクトを参照する際に MDS のリーダーノードを参照する。強整合性を保証する。
    ///
    /// 古いオブジェクトが返ってこないことが保証されるが、リーダーが決まるまでは結果が取得できない。
    /// デフォルト値。
    Consistent,
    /// オブジェクトを参照する際に過半数の MDS ノードを参照する。強整合性を保証する。
    ///
    /// 古いオブジェクトが返ってこないことが保証される。
    /// 複数ノードを参照することによるオーバーヘッドがあるため、通常は `Consistent` を利用するのが推奨される。
    Quorum,
    /// オブジェクトを参照する際に指定された数の MDS ノードを参照する。整合性は保証されない(弱整合性)。
    ///
    /// オブジェクトが更新された場合に古いバージョンのオブジェクトを返す可能性がある。
    /// 複数の異なるバージョンが取得された場合、最新のバージョンを持つオブジェクトが採用される。
    Subset(usize),
    /// 任意の1つの MDS ノードからオブジェクトを参照する。整合性は保証されない(弱整合性)。
    ///
    /// オブジェクトが更新された場合に古いデータを返す可能性がある。
    Stale,
}

impl Default for ReadConsistency {
    fn default() -> Self {
        ReadConsistency::Consistent
    }
}
