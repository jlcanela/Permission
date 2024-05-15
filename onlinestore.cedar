// allow search to product
@id("ProductSearch.session")
permit (
  principal,
  action in
    [OnlineStore::Action::"searchProduct"],
  resource
)
when { context has sessionId };
