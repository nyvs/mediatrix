#[cfg(not(feature = "async"))]
#[test]
fn atomic_test_sync() {
    use crate::synchronous::basic::*;

    use std::sync::{Arc, Mutex};

    struct IncrementRequest;
    #[derive(Debug)]
    struct IncrementEvent;

    impl RequestHandler<IncrementRequest, IncrementEvent> for BasicMediator<IncrementEvent> {
        fn handle(&self, _req: IncrementRequest) {
            self.publish(IncrementEvent)
        }
    }

    let u = Arc::new(Mutex::new(0usize));
    let cloned = u.clone();
    let mediator = BasicMediator::<IncrementEvent>::builder()
        .add_listener(move |_: &IncrementEvent| {
            let mut m = cloned.lock().unwrap();
            let c = *m;
            *m = c + 1;
        })
        .build();

    mediator.send(IncrementRequest);

    mediator.next().ok();
    assert_eq!(*(u.lock().unwrap()), 1usize);

    mediator.send(IncrementRequest);
    mediator.send(IncrementRequest);

    mediator.next().ok();
    mediator.next().ok();

    assert_eq!(*(u.lock().unwrap()), 3usize);
}

#[cfg(feature = "async")]
#[test]
fn atomic_test_async() {
    use async_trait::async_trait;
    use std::sync::{Arc, Mutex};

    use crate::asynchronous::basic::*;

    struct IncrementRequest;
    #[derive(Debug)]
    struct IncrementEvent;

    #[async_trait]
    impl AsyncRequestHandler<IncrementRequest, IncrementEvent> for BasicAsyncMediator<IncrementEvent> {
        async fn handle(&self, _req: IncrementRequest) {
            self.publish(IncrementEvent).await
        }
    }

    async_std::task::block_on(async {
        let u = Arc::new(Mutex::new(0usize));
        let cloned = u.clone();
        let async_mediator = BasicAsyncMediator::<IncrementEvent>::builder()
            .add_listener(move |_: &IncrementEvent| {
                let mut m = cloned.lock().unwrap();
                let c = *m;
                *m = c + 1;
            })
            .build();

        async_mediator.send(IncrementRequest).await;

        async_mediator.next().await.ok();
        assert_eq!(*(u.lock().unwrap()), 1usize);

        async_mediator.send(IncrementRequest).await;
        async_mediator.send(IncrementRequest).await;

        async_mediator.next().await.ok();
        async_mediator.next().await.ok();

        assert_eq!(*(u.lock().unwrap()), 3usize);
    })
}

#[cfg(feature = "async")]
#[test]
fn cxaware_mediator_atomic_test_async() {
    use async_trait::async_trait;
    use std::sync::{Arc, Mutex};

    use crate::asynchronous::contextaware::*;

    struct IncrementRequest;
    #[derive(Debug)]
    struct IncrementEvent(usize);

    let base_num: usize = 3;

    #[async_trait]
    impl CxAwareAsyncRequestHandler<usize, IncrementRequest, IncrementEvent>
        for CxAwareAsyncMediator<usize, IncrementEvent>
    {
        async fn handle(&self, _req: IncrementRequest, cx: &usize) {
            self.publish(IncrementEvent(cx.clone())).await
        }
    }

    async_std::task::block_on(async {
        let u = Arc::new(Mutex::new(0usize));
        let cloned = u.clone();
        let async_mediator = CxAwareAsyncMediator::<usize, IncrementEvent>::builder()
            .add_listener(move |x: &IncrementEvent| {
                let mut m = cloned.lock().unwrap();
                let c = *m;
                *m = c + x.0;
            })
            .add_context(base_num)
            .build()
            .unwrap();

        async_mediator.send(IncrementRequest).await;

        async_mediator.next().await.ok();
        assert_eq!(*(u.lock().unwrap()), 3usize);

        async_mediator.send(IncrementRequest).await;
        async_mediator.send(IncrementRequest).await;

        async_mediator.next().await.ok();
        async_mediator.next().await.ok();

        assert_eq!(*(u.lock().unwrap()), 9usize);
    })
}

#[cfg(feature = "async")]
#[test]
fn cxaware_mediator_atomic_arc_test_async() {
    use async_trait::async_trait;
    use std::sync::{Arc, Mutex};

    use crate::asynchronous::contextaware::*;

    struct IncrementRequest;
    #[derive(Debug)]
    struct IncrementEvent(usize);

    let base_num: Arc<Mutex<usize>> = Arc::new(Mutex::new(5));

    #[async_trait]
    impl CxAwareAsyncRequestHandler<Arc<Mutex<usize>>, IncrementRequest, IncrementEvent>
        for CxAwareAsyncMediator<Arc<Mutex<usize>>, IncrementEvent>
    {
        async fn handle(&self, _req: IncrementRequest, cx: &Arc<Mutex<usize>>) {
            let c = {
                let mut m = cx.lock().unwrap();
                *m = *m - 1;
                m.clone() + 1
            };

            self.publish(IncrementEvent(c)).await
        }
    }

    async_std::task::block_on(async {
        let u = Arc::new(Mutex::new(0usize));
        let cloned = u.clone();
        let async_mediator = CxAwareAsyncMediator::<Arc<Mutex<usize>>, IncrementEvent>::builder()
            .add_listener(move |x: &IncrementEvent| {
                let mut m = cloned.lock().unwrap();
                let c = *m;
                *m = c + x.0;
            })
            .add_context(base_num)
            .build()
            .unwrap();

        async_mediator.send(IncrementRequest).await;

        async_mediator.next().await.ok();
        assert_eq!(*(u.lock().unwrap()), 5usize);

        async_mediator.send(IncrementRequest).await;
        async_mediator.send(IncrementRequest).await;

        async_mediator.next().await.ok();
        async_mediator.next().await.ok();

        assert_eq!(*(u.lock().unwrap()), 12usize);
    })
}
