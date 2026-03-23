import http from 'k6/http';
import { check } from 'k6';

export const options = {
    scenarios: {
        million_requests: {
            executor: 'shared-iterations',
            vus: 200,              // 200 usuários metralhando simultaneamente
            iterations: 1000000,   // Exatamente 1 milhão de requisições
            maxDuration: '10m',    // Dá um limite máximo de 10 minutos para o teste terminar
        },
    },
};

export default function () {
    const url = 'http://localhost:8080/data';
    
    const payload = JSON.stringify({
        user_id: `user_${__VU}_${__ITER}`,
        action: 'compra_massiva',
        timestamp: Date.now(),
    });

    const params = {
        headers: {
            'Content-Type': 'application/json',
        },
    };

    const res = http.post(url, payload, params);

    check(res, {
        'status is 202': (r) => r.status === 202,
    });
}