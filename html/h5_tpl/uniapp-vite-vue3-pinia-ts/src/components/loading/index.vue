<template>
    <div class="contain bg-white h-screen fixed top-0 bottom-0 right-0 left-0">
        <div class="loader">
            <span class="loader-text">loading</span>
            <span class="load"></span>
        </div>
    </div>
</template>

<style lang="scss" scoped>
.contain {
    z-index: 99;
    display: flex;
    justify-content: center;
    align-items: center;
}

.loader {
    position: relative;
    width: 80px;
    height: 50px;
}

.loader-text {
    position: absolute;
    top: 0;
    margin: 0;
    padding: 0;
    font-size: 0.8rem;
    color: #c8b6ff;
    animation: text713 3.5s ease both infinite;
    letter-spacing: 1px;
}

.load {
    position: absolute;
    bottom: 0;
    display: block;
    width: 16px;
    height: 16px;
    border-radius: 50px;
    background-color: #9a79ff;
    transform: translateX(64px);
    animation: loading713 3.5s ease both infinite;
}

.load::before {
    position: absolute;
    width: 100%;
    height: 100%;
    border-radius: inherit;
    background-color: #d1c2ff;
    content: '';
    animation: loading2713 3.5s ease both infinite;
}

@keyframes text713 {
    0% {
        letter-spacing: 1px;
        transform: translateX(0);
    }

    40% {
        letter-spacing: 2px;
        transform: translateX(26px);
    }

    80% {
        letter-spacing: 1px;
        transform: translateX(32px);
    }

    90% {
        letter-spacing: 2px;
        transform: translateX(0);
    }

    100% {
        letter-spacing: 1px;
        transform: translateX(0);
    }
}

@keyframes loading713 {
    0% {
        width: 16px;
        transform: translateX(0);
    }

    40% {
        width: 100%;
        transform: translateX(0);
    }

    80% {
        width: 16px;
        transform: translateX(64px);
    }

    90% {
        width: 100%;
        transform: translateX(0);
    }

    100% {
        width: 16px;
        transform: translateX(0);
    }
}

@keyframes loading2713 {
    0% {
        transform: translateX(0);
        width: 16px;
    }

    40% {
        transform: translateX(0%);
        width: 80%;
    }

    80% {
        width: 100%;
        transform: translateX(0);
    }

    90% {
        width: 80%;
        transform: translateX(15px);
    }

    100% {
        transform: translateX(0);
        width: 16px;
    }
}
</style>
